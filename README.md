# Todo-TUI (tdt) 🚀

> Um gerenciador de tarefas para terminal (TUI) ultrarrápido, inspirado no Lazygit, com suporte a revisões espaçadas (SRS) e sincronização.

![Todo-TUI Demo](https://raw.githubusercontent.com/pfTheTrial/todo-tui/master/demo.gif) *(Adicione um .gif ou print real aqui no GitHub depois)*

## 📦 Instalação

Você pode rodar diretamente via `npx` (caso tenha NodeJS) ou baixar o binário nativo nas [Releases](../../releases).

```bash
# Instalar globalmente via NPM
npm install -g todo-tui

# E para abrir o app no terminal de qualquer lugar, basta digitar:
tdt

# Ou via cargo (se tiver Rust instalado)
cargo install --git https://github.com/pfTheTrial/todo-tui
```

## ✨ Funcionalidades

| Ferramenta | Status | Descrição |
| :--- | :---: | :--- |
| **I18n Integrado** | ✅ | Auto-detecta idioma do sistema (`PT-BR`, `EN`, `ES`). |
| **Smart Reviews** | ✅ | Adicione planos de revisão espaçada (ex: `1d 1w 1m`). |
| **Pomodoro** | ✅ | Timer natively integrado e focado nas tarefas. |
| **Notion Sync** | ✅ | Envio e sincronismo de progresso com banco de dados no Notion. |
| **Export/Import** | ✅ | Suporte para salvar e ler backups como `.xlsx` em Excel. |
| **Temas Dinâmicos** | ✅ | Suporte a Dracula, Ayu Dark, Minimal etc. |
| **Notificações OS** | ✅ | Alertas em background rodando nativamente no sistema (Desk). |
| **Integração Google Cal.**| 🔨 Desenvolvendo | Conectar *due dates* ao Agenda. |
| **Proton/GDrive Sync** | 🔨 Desenvolvendo | Backups automáticos isolados na nuvem. |
| **Modo Vim** | 🔨 Desenvolvendo | Bindings avançados estilo (hjkl). |

## ⌨️ Atalhos Essenciais da Interface

A interface é guiada por painéis (Tarefas, Detalhes, Pomodoro).

| Comando / Atalho | Ação Executada |
| :--- | :--- |
| `a` | **Add Wizard** para nova criação de tarefa. |
| `Space` | Concluí uma tarefa selecionada (Marcando review, se houver). |
| `x` | Deletar tarefa (com prompt de confirmação). |
| `e` / `d` / `t` / `r` | Editar: Título / Descrição / Data / Revisões. |
| `/` | Filtro dinâmico para buscar texto instantaneamente. |
| `o` | Alterar ordem de ordenação (Prioridade, Data, Nome). |
| `[1, 2, 3]` ou `Tab` | Alternar entre os painéis Ativos. |
| `p` ou `S` | Iniciar / Parar / Pular status do **Pomodoro**. |
| `c` | Abrir Menu de Configurações avançadas. |
| `?` | Abrir Overlay de Ajuda completa. |

## 💡 Exemplos Práticos de Uso

O fluxo de criação é extremamente rápido e permite agendar revisões automáticas no mesmo instante!

1. **Criação de Tarefa Rápida:**
   - Aperte `a` (Add).
   - Digite o título: `Finalizar Relatório Q3`.
   - Preencha os detalhes ou aperte `Enter` para pular.

2. **Agendamentos Naturais (Datas):**
   No campo de "Due Date" (Aperte `t`), você pode usar atalhos como:
   - `today`, `tomorrow` (Hoje ou Amanhã)
   - `1d`, `3w`, `2m` (Daqui a X dias, semanas ou meses)
   - `15/04/2026` (Data exata)

3. **Revisões Espaçadas (SRS):**
   Ao focar nos estudos ou revisão de código, aperte `r` para definir o padrão de repetição:
   - `1d 1w 1m`: A tarefa voltará a ficar "Pendente/Atrasada" amanhã, depois uma semana seguida do momento de término, e por fim daqui a um mês.
   - Perfeito para memorização ou follow-up com clientes!

## ⚙️ Compilação Manual e Roadmap

Para desenvolvedores locais usando a engine em Rust:

```bash
git clone https://github.com/pfTheTrial/todo-tui.git
cd todo-tui
cargo build --release
```

Os novos mantenedores podem checar as descrições atômicas de commit e acompanhar o andamento dos itens em "Desenvolvendo".
