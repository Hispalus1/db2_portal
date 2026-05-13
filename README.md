# DB2 Portal (Oracle Database Integration)

Tento projekt je semestrální práce zaměřená na integraci Rustu s Oracle databází pro správu uživatelů, her a knihoven.

## Požadavky

Pro spuštění aplikace je nutné mít nainstalovány následující komponenty:

1. **Rust Toolchain**: [Návod k instalaci](https://www.rust-lang.org/tools/install)
2. **Oracle Instant Client**: Knihovny pro připojení k Oracle databázi.

---

## Instalace a nastavení

### Linux (Ubuntu/Debian)

1. **Instalace Rustu:**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   ```

2. **Instalace Oracle Instant Client:**
   - Stáhněte "Basic Light" package z [Oracle webu](https://www.oracle.com/database/technologies/instant-client/linux-x86-64-downloads.html).
   - Rozbalte archiv (např. do `/opt/oracle/instantclient`).
   - Nainstalujte `libaio1` (nutné pro Oracle client):
     ```bash
     sudo apt-get install libaio1
     ```
   - Nastavte cestu ke knihovnám:
     ```bash
     export LD_LIBRARY_PATH=/opt/oracle/instantclient:$LD_LIBRARY_PATH
     ```

### Windows

1. **Instalace Rustu:**
   - Stáhněte a spusťte [rustup-init.exe](https://rustup.rs/).

2. **Instalace Oracle Instant Client:**
   - Stáhněte "Basic Light" package z [Oracle webu](https://www.oracle.com/database/technologies/instant-client/winx6464-downloads.html).
   - Rozbalte archiv (např. do `C:\oracle\instantclient`).
   - Přidejte cestu k rozbalené složce do systémové proměnné **PATH**.
   - (Volitelně) Může být vyžadováno "Microsoft Visual C++ Redistributable".

---

## Jak spustit projekt

1. **Klonování/Stažení projektu:**
   Vstupte do složky projektu v terminálu.

2. **Sestavení a spuštění:**
   ```bash
   cargo run
   ```

## Struktura projektu

- `src/main.rs`: Vstupní bod aplikace, konfigurace připojení.
- `src/orm/`: Obsahuje DAO (Data Access Objects) a DTO (Data Transfer Objects) pro práci s entitami:
  - `AppUser`: Správa uživatelů a peněženky.
  - `Game`: Katalog her.
  - `Library`: Vlastněné hry uživateli.
  - `Review`: Recenze her.
  - `Transactions`: Komplexní operace (nákup hry, uložení recenze).

## Poznámka k připojení
Aplikace se připojuje k univerzitní databázi na hostiteli `bayer.cs.vsb.cz`. Přihlašovací údaje jsou aktuálně nastaveny v `main.rs`. Pro úspěšné spuštění mimo univerzitní síť může být vyžadováno připojení přes **VPN**.
