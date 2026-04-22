<div align="center">

```
  ████████╗██████╗ ████████╗
  ╚══██╔══╝██╔══██╗╚══██╔══╝
     ██║   ██║  ██║   ██║   
     ██║   ██║  ██║   ██║   
     ██║   ██████╔╝   ██║   
     ╚═╝   ╚═════╝    ╚═╝   
```

**Terminal Task Dashboard (tdt)** — Gerenciador de tarefas para terminal ultrarrápido

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-stable-orange.svg)](https://www.rust-lang.org)
[![Version](https://img.shields.io/badge/version-1.0.2-blue.svg)](https://github.com/pfTheTrial/todo-tui/releases)

<img src="docs/demo.gif" width="800" alt="tdt demo animado mostrando tarefas, pomodoro e configurações reativas"/>

🇧🇷 Português | [🇺🇸 English](#english)

</div>

---

## 🇧🇷 Português

### 📖 Sobre

**tdt** é um gerenciador de tarefas para terminal inspirado no Lazygit — rápido, minimalista e poderoso. Funciona 100% offline, com dados armazenados localmente em JSON. Vem com:

- **Revisões espaçadas (SRS)** para nunca esquecer uma tarefa importante
- **Timer Pomodoro** integrado com múltiplos perfis
- **Sincronização com Notion** para equipes (auto-provisioning de colunas)
- **Sincronização com Syncthing** para multi-dispositivo (hot-reload silencioso)
- **I18n** com auto-detecção do idioma do sistema (PT-BR, EN, ES)
- **7 temas visuais** premium
- **Setup Wizard** interativo na primeira execução

---

### 📦 Instalação

```bash
# Via NPM / PNPM (instala globalmente e baixa o binário automaticamente)
npm install -g todo-tui
pnpm add -g todo-tui

# Via NPX (testar sem instalar)
npx todo-tui

# Via Shell Installer (Windows - Powershell)
iwr -useb https://raw.githubusercontent.com/pfTheTrial/todo-tui/main/install.ps1 | iex

# Via Shell Installer (Linux / macOS)
curl -fsSL https://raw.githubusercontent.com/pfTheTrial/todo-tui/main/install.sh | sh

# Via Cargo (se você tem Rust instalado)
cargo install --git https://github.com/pfTheTrial/todo-tui

# Ou baixe o binário direto das Releases:
# https://github.com/pfTheTrial/todo-tui/releases
```

**Plataformas suportadas:** Windows (x64), macOS (Intel/Apple Silicon), Linux (x64/ARM), Android (Termux)

---

### 🚀 Início Rápido

```
tdt          # Abre o app
tdt --setup  # Abre o Setup Wizard (ou re-configura)
tdt --help   # Mostra todos os comandos
```

Na primeira execução, você verá um **Setup Wizard** interativo para configurar idioma, tema e integrações.

---

### 🖥️ Comandos CLI

```
USAGE:
    tdt [OPTIONS] [COMMAND]

COMMANDS:
    update          Verifica e instala a última versão
    sync            Força um sync manual e sai
    reset           Reseta configurações (preserva tarefas)

OPTIONS:
    -h, --help              Mostra a ajuda completa
    -V, --version           Mostra a versão atual
        --setup             Abre o Setup Wizard
        --lang <LANG>       Força idioma temporário (pt-br, en, es)
        --data-dir <PATH>   Diretório de dados customizado
        --no-sync           Desativa sync automático nesta sessão
```

---

### ⌨️ Atalhos de Teclado

#### Navegação Global

| Atalho | Ação |
|--------|------|
| `Tab` / `h` / `l` | Alternar painel |
| `1` | Ir para painel Tarefas |
| `2` | Ir para painel Pomodoro |
| `3` | Ir para painel Detalhes |
| `?` | Abrir/fechar ajuda |
| `q` | Sair |

#### Painel de Tarefas (`[1]`)

| Atalho | Ação |
|--------|------|
| `j` / `↓` | Próxima tarefa |
| `k` / `↑` | Tarefa anterior |
| `g` / `G` | Ir para início / fim da lista |
| `a` | **Adicionar** tarefa (wizard 4 passos) |
| `Space` | Marcar tarefa como concluída / avançar revisão |
| `Backspace` | Desfazer conclusão |
| `x` | Deletar tarefa (com confirmação) |
| `e` | Editar título da tarefa |
| `d` | Editar descrição |
| `t` | Editar data / prazo |
| `r` | Editar plano de revisão SRS |
| `i` | Ciclar importância (Baixa → Média → Alta → Urgente) |
| `o` | Ciclar ordenação (Prioridade → Data → Nome) |
| `/` | Buscar / filtrar tarefas |

#### Painel Pomodoro (`[2]`)

| Atalho | Ação |
|--------|------|
| `p` | Play / Pause |
| `S` | Pular fase atual |
| `R` | Resetar timer |
| `f` | Forçar pausa |
| `j` / `k` | Trocar perfil de pomodoro |
| `e` | Editar perfil atual (`nome trabalho pausa_curta pausa_longa`) |

#### Painel Detalhes (`[3]`)

| Atalho | Ação |
|--------|------|
| `j` / `k` | Rolar conteúdo da descrição |
| `J` / `K` | Navegar entre tarefas |
| `e` | Editar descrição |
| `r` | Editar revisão |

#### Menus e Modos

| Atalho | Ação |
|--------|------|
| `c` | Abrir Configurações |
| `Enter` | Confirmar entrada |
| `Esc` | Cancelar / voltar |

---

### 💡 Exemplos Práticos

#### 1. Criar uma Tarefa com Revisão

```
Pressione "a" → wizard abre:

  Título:      > Estudar Rust ownership
  Descrição:   > Capítulos 4-6 do The Book
  Data/Prazo:  > 3d          ← daqui a 3 dias
  Revisão SRS: > 1d 1s 1m    ← revisar em 1 dia, 1 semana, 1 mês
```

#### 2. Formatos de Data Aceitos

| Entrada | Resultado |
|---------|-----------|
| `hoje` / `today` / `hoy` | Hoje |
| `amanha` / `tomorrow` / `mañana` | Amanhã |
| `3d` | Daqui a 3 dias |
| `2s` / `2w` | Daqui a 2 semanas |
| `1m` | Daqui a 1 mês |
| `sex` / `fri` | Próxima sexta-feira |
| `15/04` | 15 de abril (ano atual) |
| `15/04/2027` | Data exata |

#### 3. Plano de Revisão SRS

```
Campo "Revisão": 1d 1s 1m 3m

Isso cria 4 subtarefas:
  [ ] Review 1 — 1 dia depois da data de conclusão
  [ ] Review 2 — 1 semana depois
  [ ] Review 3 — 1 mês depois
  [ ] Review 4 — 3 meses depois

A tarefa é deletada automaticamente quando todas as revisões forem concluídas.
```

#### 4. Perfis Personalizados do Pomodoro

```
Selecione um perfil, pressione "e":

  Entrada: "DeepWork 90 15 30"
           ─────── ── ── ──
           nome   W  S   L   (minutos)
```

---

### ✨ Funcionalidades

| Recurso | Status | Descrição |
|---------|--------|-----------|
| **I18n Auto-detect** | ✅ | Detecta idioma do sistema (PT-BR, EN, ES) |
| **Revisões SRS** | ✅ | Planos de revisão espaçada personalizáveis |
| **Pomodoro** | ✅ | Timer com múltiplos perfis configuráveis |
| **7 Temas** | ✅ | Dracula, Catppuccin, Nord, Gruvbox, Tokyo Night, Solarized, Minimal |
| **Notificações OS** | ✅ | Alertas nativos do sistema operacional |
| **Update Check & Auto-update** | ✅ | Verifica atualizações e instala a nova versão *in-place* no binário |
| **Setup Wizard** | ✅ | Tour guiado na primeira execução com animações |
| **Live Resource Monitor** | ✅ | Visualização de telemetria base de RAM e CPU no painel settings |
| **CLI Completa** | ✅ | Flags, subcomandos e help integrado |
| **Notion Sync** | 🔨 | Sincroniza tarefas com banco Notion (auto-provisioning) |
| **Syncthing** | 🔨 | Status de sincronização + hot-reload silencioso |
| **Schema Versioning** | 🔨 | Migração automática de dados entre versões |
| **Shell Installer** | 🔨 | Instalador one-liner para Windows/Linux/macOS |
| **NPM Distribution** | 🔨 | Pacote `todo-tui` para npm/pnpm/npx |

---

### 🎨 Temas Disponíveis

| Tema | Descrição |
|------|-----------|
| `Dracula` | Dark purple — o padrão |
| `Catppuccin` | Dark mocha com azuis suaves |
| `Nord` | Azuis árticos gelados |
| `Gruvbox` | Earth tones quentes |
| `Tokyo Night` | Neon sobre fundo escuro |
| `Solarized` | Clássico com contraste suave |
| `Minimal` | Sem cores fixas — usa o tema do seu terminal |

Para trocar o tema: `c` (Configurações) → selecione **Tema** → `Enter`

---

### 🌐 Idiomas Suportados

| Idioma | Código | Auto-detecção |
|--------|--------|---------------|
| Português (BR) | `PT-BR` | `pt_BR`, `pt` |
| English | `EN` | Qualquer outro locale |
| Español | `ES` | `es_*` |

---

### ⚙️ Compilação Manual

```bash
git clone https://github.com/pfTheTrial/todo-tui.git
cd todo-tui
cargo build --release
# Binário em: target/release/tdt
```

**Requisitos:** Rust stable (1.70+)

---

### 🗺️ Roadmap

- [x] Core TUI com layout Lazygit
- [x] Sistema de Revisão SRS
- [x] Pomodoro com perfis
- [x] I18n PT-BR / EN / ES
- [x] 7 Temas visuais premium
- [x] Notificações nativas
- [x] In-place Auto-update inteligente do binário
- [x] Otimização e Live Telemetry (RAM/CPU Dashboard)
- [x] Setup Wizard com animações
- [ ] Integração Notion (auto-provisioning de colunas)
- [ ] Integração Syncthing (status + hot-reload)
- [ ] CLI completa com `clap` (flags, subcomandos)
- [ ] Schema versioning + backup automático
- [ ] Distribuição NPM (`todo-tui`)
- [ ] Shell Installer (Powershell/Bash)
- [ ] Suporte Android (Termux)

---

### 📄 Licença

[MIT](LICENSE) © 2026 pfTheTrial

---
---

<div align="center" id="english">

## 🇺🇸 English

</div>

### 📖 About

**tdt** is a blazing-fast terminal task manager inspired by Lazygit — minimal, powerful, and 100% offline. Data is stored locally as JSON. Features include:

- **Spaced Repetition System (SRS)** so important tasks never fall through the cracks
- **Built-in Pomodoro timer** with multiple profiles
- **Notion sync** for team workflows (auto-provisioning of database columns)
- **Syncthing integration** for multi-device sync (silent hot-reload)
- **I18n** with automatic system locale detection (PT-BR, EN, ES)
- **7 premium visual themes**
- **In-place Auto-update** replacing the executable seamlessly
- **Live Telemetry** checking its ultra-lightweight RAM/CPU usage directly in the TUI Settings
- **Interactive Setup Wizard** on first run

---

### 📦 Installation

```bash
# Via NPM / PNPM (installs globally and downloads the correct binary automatically)
npm install -g todo-tui
pnpm add -g todo-tui

# Via NPX (try without installing)
npx todo-tui

# Via Shell Installer (Windows - Powershell)
iwr -useb https://raw.githubusercontent.com/pfTheTrial/todo-tui/main/install.ps1 | iex

# Via Shell Installer (Linux / macOS)
curl -fsSL https://raw.githubusercontent.com/pfTheTrial/todo-tui/main/install.sh | sh

# Via Cargo (if you have Rust installed)
cargo install --git https://github.com/pfTheTrial/todo-tui

# Or download the binary from Releases:
# https://github.com/pfTheTrial/todo-tui/releases
```

**Supported Platforms:** Windows (x64), macOS (Intel/Apple Silicon), Linux (x64/ARM), Android (Termux)

---

### 🚀 Quick Start

```
tdt          # Launch the app
tdt --setup  # Open Setup Wizard (or re-configure)
tdt --help   # Show all commands
```

On first launch, an **interactive Setup Wizard** guides you through language, theme, and integrations.

---

### 🖥️ CLI Commands

```
USAGE:
    tdt [OPTIONS] [COMMAND]

COMMANDS:
    update          Check and install latest version
    sync            Force manual sync and exit
    reset           Reset settings (preserves tasks)

OPTIONS:
    -h, --help              Print help information
    -V, --version           Print version
        --setup             Open the Setup Wizard
        --lang <LANG>       Override language (pt-br, en, es)
        --data-dir <PATH>   Custom data directory
        --no-sync           Disable auto-sync for this session
```

---

### ⌨️ Keyboard Shortcuts

#### Global Navigation

| Key | Action |
|-----|--------|
| `Tab` / `h` / `l` | Switch panel |
| `1` | Jump to Task list panel |
| `2` | Jump to Pomodoro panel |
| `3` | Jump to Detail panel |
| `?` | Toggle help overlay |
| `q` | Quit |

#### Task List Panel (`[1]`)

| Key | Action |
|-----|--------|
| `j` / `↓` | Next task |
| `k` / `↑` | Previous task |
| `g` / `G` | Jump to top / bottom |
| `a` | **Add** task (4-step wizard) |
| `Space` | Complete task / advance review |
| `Backspace` | Undo completion |
| `x` | Delete task (with confirmation) |
| `e` | Edit title |
| `d` | Edit description |
| `t` | Edit due date |
| `r` | Edit SRS review plan |
| `i` | Cycle importance (Low → Medium → High → Urgent) |
| `o` | Cycle sort (Priority → Date → Name) |
| `/` | Search / filter tasks |

#### Pomodoro Panel (`[2]`)

| Key | Action |
|-----|--------|
| `p` | Play / Pause |
| `S` | Skip current phase |
| `R` | Reset timer |
| `f` | Force a break |
| `j` / `k` | Switch Pomodoro profile |
| `e` | Edit profile (`name work short_break long_break`) |

#### Detail Panel (`[3]`)

| Key | Action |
|-----|--------|
| `j` / `k` | Scroll description |
| `J` / `K` | Navigate tasks |
| `e` | Edit description |
| `r` | Edit review plan |

---

### 💡 Practical Examples

#### 1. Creating a Task with Reviews

```
Press "a" → wizard opens:

  Title:       > Study Rust ownership
  Description: > Chapters 4-6 of The Book
  Due date:    > 3d           ← 3 days from now
  SRS Review:  > 1d 1w 1m    ← review in 1 day, 1 week, 1 month
```

#### 2. Accepted Date Formats

| Input | Result |
|-------|--------|
| `today` / `hoje` / `hoy` | Today |
| `tomorrow` / `amanha` / `mañana` | Tomorrow |
| `3d` | 3 days from now |
| `2w` / `2s` | 2 weeks from now |
| `1m` | 1 month from now |
| `fri` / `sex` | Next Friday |
| `15/04` | April 15 (current year) |
| `15/04/2027` | Exact date |

#### 3. SRS Review Plan

```
Review field: 1d 1w 1m 3m

This creates 4 subtasks anchored to the task's due date:
  [ ] Review 1 — 1 day after completion
  [ ] Review 2 — 1 week after
  [ ] Review 3 — 1 month after
  [ ] Review 4 — 3 months after

The task is automatically deleted when all reviews are completed.
```

#### 4. Custom Pomodoro Profiles

```
Select profile, press "e":

  Input: "DeepWork 90 15 30"
          ──────── ── ── ──
          name     W  S   L   (minutes)
```

---

### ✨ Features

| Feature | Status | Description |
|---------|--------|-------------|
| **Auto I18n** | ✅ | Detects system locale (PT-BR, EN, ES) |
| **SRS Reviews** | ✅ | Customizable spaced repetition plans |
| **Pomodoro** | ✅ | Timer with multiple configurable profiles |
| **7 Themes** | ✅ | Dracula, Catppuccin, Nord, Gruvbox, Tokyo Night, Solarized, Minimal |
| **OS Notifications** | ✅ | Native system notifications |
| **Auto-updater** | ✅ | Background version check + seamless in-place binary update |
| **Setup Wizard** | ✅ | Guided onboarding with animations |
| **Live Telemetry** | ✅ | Real-time RAM/CPU performance stats straight into the TUI settings |
| **Full CLI** | ✅ | Flags, subcommands, and built-in help |
| **Notion Sync** | 🔨 | Push tasks to Notion (auto-provisioning columns) |
| **Syncthing** | 🔨 | Sync status + silent hot-reload |
| **Schema Versioning** | 🔨 | Automatic data migration between versions |
| **Shell Installer** | 🔨 | One-liner installer for Windows/Linux/macOS |
| **NPM Distribution** | 🔨 | `todo-tui` package for npm/pnpm/npx |

---

### 🎨 Available Themes

| Theme | Description |
|-------|-------------|
| `Dracula` | Dark purple — default |
| `Catppuccin` | Dark mocha with soft blues |
| `Nord` | Icy arctic blues |
| `Gruvbox` | Warm earth tones |
| `Tokyo Night` | Neon on deep dark |
| `Solarized` | Classic with gentle contrast |
| `Minimal` | No fixed colors — uses your terminal theme |

Switch: `c` (Settings) → **Theme** → `Enter`

---

### 🌐 Supported Languages

| Language | Code | Auto-detection |
|----------|------|----------------|
| Português (BR) | `PT-BR` | `pt_BR`, `pt` |
| English | `EN` | Any other locale |
| Español | `ES` | `es_*` |

---

### ⚙️ Building from Source

```bash
git clone https://github.com/pfTheTrial/todo-tui.git
cd todo-tui
cargo build --release
# Binary at: target/release/tdt
```

**Requirements:** Rust stable (1.70+)

---

### 🗺️ Roadmap

- [x] Core TUI with Lazygit-style layout
- [x] SRS review system
- [x] Pomodoro with profiles
- [x] I18n PT-BR / EN / ES
- [x] 7 premium visual themes
- [x] Native OS notifications
- [x] In-place auto-updater and background startup check
- [x] Live TUI telemetry (RAM/CPU metrics)
- [x] Setup Wizard with animations
- [ ] Notion integration (auto-provisioning columns)
- [ ] Syncthing integration (status + hot-reload)
- [ ] Full CLI with `clap` (flags, subcommands)
- [ ] Schema versioning + automatic backups
- [ ] NPM distribution (`todo-tui`)
- [ ] Shell Installer (Powershell/Bash)
- [ ] Android support (Termux)

---

### 📄 License

[MIT](LICENSE) © 2026 pfTheTrial
