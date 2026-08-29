# 📖 Sorayunara Language Reference Manual (Syntax v2.1)
**Language Specification**: Sorayunara v2.1-LTS · **Extension**: `.sora`  
**Reference Document**: `Sorayunara_Syntax_Tambahan_v2_1.pdf` & `Sorayunara_Syntax_v2_Design_Proposal.pdf`

---

## 1. Core Keywords & Shorthands

### 1.1 Core Keywords
- **Bindings & Functions**: `let`, `var`, `fn`, `type`, `enum`, `trait`, `impl`, `extend`, `newtype`
- **Control Flow**: `if`, `else`, `match`, `guard`, `for`, `in`, `loop`, `break`, `continue`, `return`, `err`
- **Modules & Access**: `use`, `mod`, `pub`, `as`
- **Concurrency & Actors**: `async`, `await`, `spawn`, `actor`, `on`, `defer`, `unsafe`, `where`
- **Literals & Output**: `true`, `false`, `nil`, `say`

### 1.2 Recommended Builtin Macros & Context
`with`, `test`, `expect`, `assert`, `dbg`, `todo`, `unreachable`

---

## 2. Syntax v2.1 Ergonomic Features

### 2.1 Default & Named Parameters
```sora
fn connect(host = "localhost", port = 8080, secure = true) {
    // ...
}

connect()
connect(port: 3000)
connect(host: "api.sora", port: 443)
```

### 2.2 Short Struct Construction & Immutable Updates
```sora
type User {
    name: String,
    age: Int
}

let name = "Lutfi"
let age = 21
let user = User { name, age }          // Shorthand field init
let older = user { age: 22 }           // Immutable field update
```

### 2.3 Tuple, Destructuring & Ignore
```sora
let point = (10, 20)
let (x, y) = point
let (name, _, email) = account
let {id, name} = user
```

### 2.4 Range Expressions & Comprehensions
- `0..10` (exclusive, 0 to 9)
- `0..=10` (inclusive, 0 to 10)
- `10..0 step -1` (descending range)
- Collection Comprehensions:
  ```sora
  let squares = [x * x for x in 1..=10]
  let adults = [u for u in users if u.age >= 18]
  ```

### 2.5 Guard Clauses
```sora
fn withdraw(balance: Int, amount: Int) -> Result<Int, String> {
    guard amount > 0 else err "amount must be positive"
    guard balance >= amount else err "insufficient balance"
    balance - amount
}
```

### 2.6 Option, Null Coalescing & Optional Chaining
```sora
let name = user?.profile?.name ?? "Anonymous"
if let email = user?.email {
    send(email)
}
let upper = user?.name |> ?.upper()
```

### 2.7 Actor Ergonomics & Mailbox Handlers
```sora
actor Counter {
    var value = 0
    on Inc(n = 1) => value += n
    on Get(reply) => reply(value)
}

let counter = Counter()
counter ! Inc(5)
let value = await counter ? Get
```

### 2.8 Channel Operations
```sora
ch <- Message("hello")      // Send shorthand
let msg = <-ch              // Receive shorthand
```

### 2.9 Resource Scope (`with`) & Defer
```sora
with file = open("data.txt") {
    say file.read()
}

defer db.close()
```
