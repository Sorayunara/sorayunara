# Sorayunara プログラミング言語 (`.sora`)

<p align="center">
  [English](../../README.md) •
  [简体中文](zh.md) •
  [हिन्दी](hi.md) •
  [Español](es.md) •
  [Français](fr.md) •
  [العربية](ar.md) •
  [বাংলা](bn.md) •
  [Português](pt.md) •
  [Русский](ru.md) •
  [اردو](ur.md) •
  [Bahasa Indonesia](id.md) •
  [Deutsch](de.md) •
  **日本語** •
  [Nigerian Pidgin](pcm.md) •
  [मराठी](mr.md)
</p>


> **Hindley-Milner 型推論、フロー検知ボローチェッカー、ロックフリー Actor 並行処理、およびネイティブ LLVM/WASM/C コード生成を備えた次世代システム＆バックエンド言語。**

---

## ⚡ Sorayunara クイックルック

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

## 🏛️ 主要アーキテクチャの柱

1. **Hindley-Milner 型システム**: 冗長な記述を排除した型推論と網羅的パターンマッチング。
2. **GC 不要のメモリ安全性**: コンパイル時ボローチェックによるデータ競合の完全排除。
3. **Actor 並行処理モデル**: 軽量タスクと MPSC チャンネルによる安全なメッセージ通信。
4. **マルチターゲットコード生成**: LLVM ネイティブ、WebAssembly (WASM)、C99 出力。
5. **統合ツールチェーン**: LSP サーバー、DAP デバッガー、フォーマッター、パッケージマネージャー完備。

---

## 🛠️ ツールチェーン CLI コマンド

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

*ライセンス: MIT • [GitHub Repository](https://github.com/Sorayunara/sorayunara)*
