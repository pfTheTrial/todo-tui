use crate::model::Task;
use rust_xlsxwriter::*;
use chrono::Local;
use std::path::PathBuf;

pub fn export_tasks(tasks: &[Task]) -> Result<String, Box<dyn std::error::Error>> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();
    worksheet.set_name("Tarefas")?;

    // Header format
    let header_format = Format::new()
        .set_bold()
        .set_background_color(Color::RGB(0x44475A))
        .set_font_color(Color::RGB(0xF8F8F2))
        .set_border(FormatBorder::Thin);

    // Headers
    let headers = ["Título", "Descrição", "Data", "Importância", "Status", "Plano de Revisão", "Revisões Completas"];
    for (col, header) in headers.iter().enumerate() {
        worksheet.write_string_with_format(0, col as u16, *header, &header_format)?;
    }

    // Column widths
    worksheet.set_column_width(0, 30)?;  // Title
    worksheet.set_column_width(1, 40)?;  // Description
    worksheet.set_column_width(2, 15)?;  // Date
    worksheet.set_column_width(3, 12)?;  // Importance
    worksheet.set_column_width(4, 12)?;  // Status
    worksheet.set_column_width(5, 25)?;  // Review plan
    worksheet.set_column_width(6, 20)?;  // Reviews done

    let urgent_fmt = Format::new().set_font_color(Color::RGB(0xFF5555));
    let high_fmt = Format::new().set_font_color(Color::RGB(0xFFB86C));
    let medium_fmt = Format::new().set_font_color(Color::RGB(0xF1FA8C));
    let done_fmt = Format::new().set_font_color(Color::RGB(0x50FA7B));

    for (row, task) in tasks.iter().enumerate() {
        let r = (row + 1) as u32;
        
        worksheet.write_string(r, 0, &task.title)?;
        worksheet.write_string(r, 1, &task.description)?;
        
        let date_str = match task.due_date {
            Some(dt) => dt.with_timezone(&Local).format("%d/%m/%Y").to_string(),
            None => "Sem data".to_string(),
        };
        worksheet.write_string(r, 2, &date_str)?;
        
        let (imp_str, imp_fmt) = match task.importance {
            crate::model::task::Importance::Urgent => ("Urgente", Some(&urgent_fmt)),
            crate::model::task::Importance::High => ("Alta", Some(&high_fmt)),
            crate::model::task::Importance::Medium => ("Média", Some(&medium_fmt)),
            crate::model::task::Importance::Low => ("Baixa", None),
        };
        if let Some(fmt) = imp_fmt {
            worksheet.write_string_with_format(r, 3, imp_str, fmt)?;
        } else {
            worksheet.write_string(r, 3, imp_str)?;
        }

        let status = if task.completed {
            if task.review_subtasks.iter().all(|s| s.completed) { "Concluída" } else { "Em Revisão" }
        } else { "Pendente" };
        if task.completed {
            worksheet.write_string_with_format(r, 4, status, &done_fmt)?;
        } else {
            worksheet.write_string(r, 4, status)?;
        }
        
        worksheet.write_string(r, 5, &task.custom_review_str)?;
        
        let done_count = task.review_subtasks.iter().filter(|s| s.completed).count();
        let total = task.review_subtasks.len();
        if total > 0 {
            worksheet.write_string(r, 6, &format!("{}/{}", done_count, total))?;
        }
    }

    // Save to user-chosen path (default: Desktop)
    let desktop = dirs::desktop_dir().unwrap_or_else(|| PathBuf::from("."));
    let filename = format!("tarefas_{}.xlsx", Local::now().format("%Y%m%d_%H%M%S"));
    let path = desktop.join(&filename);
    
    workbook.save(&path)?;
    
    Ok(path.to_string_lossy().to_string())
}

use calamine::{Reader, Xlsx, RangeDeserializerBuilder};

pub fn import_tasks_from_xlsx(path: &str) -> Result<Vec<Task>, Box<dyn std::error::Error>> {
    let mut excel: Xlsx<_> = calamine::open_workbook(path)?;
    let sheet_name = excel.sheet_names().get(0).ok_or("Nenhuma planilha encontrada")?.clone();
    let range = excel.worksheet_range(&sheet_name)?;
    
    let mut tasks = Vec::new();
    let iter = RangeDeserializerBuilder::new().from_range(&range)?;
    
    for result in iter {
        let (title, description, date, importance, _status, review_plan, _done): (String, String, String, String, String, String, String) = result?;
        let mut task = Task::new(title, description);
        task.due_date = crate::utils::date_parser::parse_date_input(&date);
        task.importance = match importance.to_lowercase().as_str() {
            "urgente" | "urgent" => crate::model::task::Importance::Urgent,
            "alta" | "high"      => crate::model::task::Importance::High,
            "m\u{00e9}dia" | "media" | "medium" => crate::model::task::Importance::Medium,
            _ => crate::model::task::Importance::Low,
        };
        task.custom_review_str = review_plan;
        task.generate_review_subtasks();
        tasks.push(task);
    }
    
    Ok(tasks)
}
