# Le Langage de Programmation Sorayunara (`.sora`)

<p align="center">
  [English](../../README.md) •
  [简体中文](zh.md) •
  [हिन्दी](hi.md) •
  [Español](es.md) •
  **Français** •
  [العربية](ar.md) •
  [বাংলা](bn.md) •
  [Português](pt.md) •
  [Русский](ru.md) •
  [اردو](ur.md) •
  [Bahasa Indonesia](id.md) •
  [Deutsch](de.md) •
  [日本語](ja.md) •
  [Nigerian Pidgin](pcm.md) •
  [मराठी](mr.md)
</p>


> **Langage système et backend nouvelle génération doté de l'inférence de types Hindley-Milner, d'un borrow checker sensible au flux, d'une concurrence par acteurs sans verrou et d'une génération de code natif LLVM / WASM / C.**

---

## ⚡ Aperçu Rapide de Sorayunara

```sora
import std.io
import std.channel

struct WorkerMessage {
    id: Int,
    payload: String
}

async fn worker_task(id: Int, ch: Channel<WorkerMessage>) -> Result<String, String> {
    print("Worker #" + id.to_string() + " running.")
    let msg = ch.recv()
    match msg {
        Option::Some(data) => {
            print("Received: " + data.payload)
            return Result::Ok("Success")
        },
        Option::None => {
            return Result::Err("Channel closed")
        }
    }
}

fn main() -> Int {
    print("Sorayunara (.sora) Runtime Active!")
    let ch: Channel<WorkerMessage> = channel::new(1024)
    spawn async {
        worker_task(1, ch)
    }
    ch.send(WorkerMessage { id: 1, payload: "Compute task" })
    return 0
}
```

---

## 🏛️ Piliers d'Architecture

1. **Système de Types Hindley-Milner**: Inférence de types sans boilerplate et filtrage par motif exhaustif.
2. **Sécurité Mémoire sans GC**: Emprunts vérifiés à la compilation pour éliminer les courses de données.
3. **Modèle d'Acteurs Concurrents**: Tâches asynchrones légères et canaux de communication MPSC.
4. **Compilation Multi-Cible**: Support natif LLVM, WebAssembly (WASM) et transpilation C99.
5. **Chaîne d'Outils Complète**: LSP, débogueur DAP, formateur et gestionnaire de paquets intégrés.

---

## 🛠️ Commandes du Toolchain

```bash
sorayunara compile main.sora    # Compile to native target
sorayunara run main.sora        # Compile & run
sorayunara build main.sora      # Build optimized release binary
sorayunara test                 # Run test suites & assert blocks
sorayunara fmt main.sora        # Format source code
sorayunara check main.sora      # Type check & borrow check
sorayunara debug main.sora      # Interactive DAP debug session
sorayunara lsp                  # Language Server Protocol daemon
```

---

*Licence: MIT • [GitHub Repository](https://github.com/Sorayunara/sorayunara)*
