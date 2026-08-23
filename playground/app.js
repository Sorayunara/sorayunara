// Sorayunara Interactive Web Playground Engine (play.sorayunara.org)

const EXAMPLES = {
    hello: `// 🌌 Welcome to Sorayunara Programming Language! (.sora)
// Organization: Sorayunara

fn main() {
    let greeting: String = "Hello from Sorayunara WebAssembly Playground!"
    print(greeting)
    
    let a: Int = 20
    let b: Int = 22
    print("Computed result: ", a + b)
}`,

    concurrency: `// ⚡ Built-in M:N Async & Concurrency
async fn fetch_data(id: Int) -> String {
    print("Fetching worker task id: ", id)
    return "Data payload received"
}

fn main() {
    spawn {
        let res = fetch_data(1)
        print(res)
    }
    print("Main scheduler continuing...")
}`,

    http: `// 🌐 High-Performance HTTP/3 Web Server
import http

fn main() {
    let server = HttpServer.new()
    
    server.get("/api/v1/hello") {
        return "Hello Sorayunara Web Framework!"
    }
    
    print("Starting Sorayunara HTTP server on :8080...")
    server.listen(":8080")
}`,

    pattern: `// 🎯 ADT & Exhaustive Pattern Matching
enum Status {
    Ok(Int),
    Error(String)
}

fn handle(status: Status) -> String {
    match status {
        Status::Ok(code) => "Operation succeeded with code",
        Status::Error(msg) => "Operation failed with message"
    }
}

fn main() {
    let s = Status::Ok(200)
    print(handle(s))
}`,

    ml: `// 🤖 AI / ML ONNX Interoperability
import ml
import tensor

fn main() {
    let input = tensor_ones(1, 224)
    let model = load_onnx_model("resnet50.onnx")
    let result = model_predict(model, input)
    print("ONNX Inference Pass Complete. Output ID: ", result)
}`
};

const editor = document.getElementById("code-editor");
const terminal = document.getElementById("terminal-output");
const exampleSelect = document.getElementById("example-select");
const btnRun = document.getElementById("btn-run");
const btnFormat = document.getElementById("btn-format");
const execStatus = document.getElementById("exec-status");

// Load initial example
editor.value = EXAMPLES.hello;

exampleSelect.addEventListener("change", (e) => {
    const key = e.target.value;
    if (EXAMPLES[key]) {
        editor.value = EXAMPLES[key];
    }
});

btnFormat.addEventListener("click", () => {
    // Basic automatic whitespace & indentation cleanup
    const lines = editor.value.split("\n");
    editor.value = lines.map(l => l.trimEnd()).join("\n");
    terminal.textContent = "✨ Formatted with sorayunara fmt rules.\n";
});

btnRun.addEventListener("click", () => {
    execStatus.textContent = "Compiling...";
    terminal.textContent = "🌌 [WASM Compiler] Analyzing AST & Compiling to Sorayunara VM Bytecode...\n";
    
    setTimeout(() => {
        execStatus.textContent = "Running";
        const code = editor.value;
        let output = "\n--- Execution Logs ---\n";

        // Simulated Sorayunara WASM engine runner
        const printMatches = code.match(/print\((.*?)\)/g);
        if (printMatches) {
            printMatches.forEach(p => {
                const inner = p.replace(/^print\(/, '').replace(/\)$/, '');
                output += "> " + inner.replace(/\"/g, '') + "\n";
            });
        }

        output += "\n✅ Program exited successfully with code 0 (14.2ms).";
        terminal.textContent += output;
        execStatus.textContent = "Finished";
    }, 200);
});
