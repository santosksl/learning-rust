## Como Funciona o Ownership em Rust?

O **ownership** (propriedade) é um dos conceitos centrais e mais poderosos do Rust. Ele é o mecanismo que permite que Rust gerencie a memória de forma segura **sem um garbage collector** (como em Java ou Python), evitando problemas como memory leaks, dangling pointers ou data races. Em resumo, o ownership garante que a memória seja alocada e liberada de forma automática e previsível, tudo verificado **em tempo de compilação** pelo compilador (borrow checker).

Vou explicar passo a passo, com exemplos simples. Vamos focar nas regras principais, moves, borrows e como isso se relaciona com stack e heap.

---

### 🔑 **As Três Regras Básicas do Ownership**

Rust impõe regras estritas para ownership. Elas são simples, mas profundas:

1. **Cada valor em Rust tem um único "dono" (owner).**  
   - Um valor (como uma variável) pertence a apenas uma variável por vez.  
   - Isso evita que múltiplos "donos" tentem liberar a mesma memória.

2. **Quando o dono sai de escopo, o valor é automaticamente liberado (dropped).**  
   - "Escopo" é o bloco `{}` onde a variável é declarada.  
   - Rust chama o método `drop` automaticamente para liberar recursos (memória, arquivos, etc.).

3. **Ownership pode ser transferido (moved) ou emprestado (borrowed).**  
   - **Move**: Transfere o ownership para outro dono (o valor original fica inválido).  
   - **Borrow**: Permite acessar o valor temporariamente sem transferir ownership (via referências `&` ou `&mut`).

Essas regras são aplicadas em **todos os tipos**: primitivos (como `i32`), structs, enums, vetores (`Vec`), strings (`String`), etc. Tipos no **stack** são copiados se implementarem `Copy` (ex: números), mas tipos no **heap** (como `String`) são movidos por padrão.

---

### 🚚 **Moves: Transferindo Ownership**

- Um **move** acontece quando você atribui uma variável a outra ou passa para uma função.  
- O ownership é transferido, e a variável original não pode mais ser usada (o compilador impede isso).  
- Isso é eficiente: sem cópias desnecessárias, especialmente para dados no heap.

**Exemplo Básico:**
```rust
fn main() {
    let s1 = String::from("hello");  // s1 é o dono da String (dados no heap)

    let s2 = s1;  // MOVE: ownership transferido para s2. s1 fica inválido!

    println!("{}", s2);  // OK
    // println!("{}", s1);  // ERRO! s1 não é mais válido (use-after-move)
}  // s2 sai de escopo → String é dropada automaticamente
```

- **Por quê?** `String` gerencia dados no heap. Copiar seria caro, então Rust move por padrão.  
- Para tipos simples como `i32` (no stack e com `Copy`), não há move — é copiado:
```rust
let x = 42;  // x no stack
let y = x;   // CÓPIA: y tem uma cópia de 42
println!("x = {}, y = {}", x, y);  // OK, ambos válidos
```

**Move em Funções:**
```rust
fn take_ownership(s: String) {  // ownership movido para 's' dentro da função
    println!("{}", s);
}  // s sai de escopo → drop

fn main() {
    let s = String::from("world");
    take_ownership(s);  // move para a função
    // println!("{}", s);  // ERRO! s movido
}
```

- Para retornar ownership: Use `return` ou passe de volta.

---

### 🔗 **Borrows: Emprestando Acesso Temporário**

- **Borrow** permite acessar dados sem transferir ownership.  
- Usa **referências**: `&T` (imutável) ou `&mut T` (mutável).  
- Regras do **borrow checker**:
  - Você pode ter **múltiplos borrows imutáveis** ao mesmo tempo (leitura segura).  
  - Ou **exatamente um borrow mutável** (escrita exclusiva, evita data races).  
  - Borrows não podem durar mais que o dono (lifetimes garantem isso).

**Exemplo de Borrow Imutável:**
```rust
fn calculate_length(s: &String) -> usize {  // borrow &String (não move)
    s.len()  // acessa sem mudar
}  // borrow acaba, mas ownership fica com o caller

fn main() {
    let s = String::from("hello");
    let len = calculate_length(&s);  // passa referência
    println!("{} tem comprimento {}", s, len);  // s ainda válido!
}
```

**Exemplo de Borrow Mutável:**
```rust
fn change(s: &mut String) {  // &mut permite mudar
    s.push_str(", world!");
}

fn main() {
    let mut s = String::from("hello");  // 'mut' necessário para &mut
    change(&mut s);
    println!("{}", s);  // "hello, world!"
}
```

- **Regra de Exclusividade:** Não misture & e &mut ao mesmo tempo:
```rust
let mut s = String::from("hello");
let r1 = &s;     // borrow imutável
let r2 = &s;     // outro imutável OK
// let r3 = &mut s;  // ERRO! Não pode mut enquanto há imutáveis
println!("{} {}", r1, r2);
```

---

### ⏳ **Lifetimes: Garantindo Validade das Referências**

- Lifetimes são anotações (`'a`) que dizem quanto tempo uma referência vive.  
- O compilador infere a maioria, mas às vezes você precisa explicitar (ex: em structs ou funções).  
- Evita "dangling references" (referências a dados já dropados).

**Exemplo Simples:**
```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {  // 'a: lifetime compartilhado
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let s1 = String::from("long");
    let s2 = "short";
    let result = longest(&s1, s2);  // OK, lifetimes compatíveis
}
```

---

### 🛡️ **Vantagens e Segurança**

- **Sem Garbage Collector:** Memória liberada deterministicamente (no drop).  
- **Thread Safety:** Ownership previne data races (ex: &mut é exclusivo).  
- **Eficiência:** Moves evitam cópias desnecessárias.  
- **Erros em Compilação:** Problemas como use-after-free são pegos cedo, não em runtime.

**Diferenças com Outras Linguagens:**
- C/C++: Gerenciamento manual (new/delete), propenso a erros.  
- Python/Java: GC automático, mas overhead e pausas.  
- Rust: Automático via ownership, zero-cost abstractions.

| Conceito          | Descrição                          | Exemplo                  |
|-------------------|------------------------------------|--------------------------|
| Owner             | Dono único do valor                | `let s = String::new();` |
| Move              | Transfere ownership                | `let s2 = s1;`           |
| Borrow Imutável   | Acesso leitura (múltiplos OK)      | `&s`                     |
| Borrow Mutável    | Acesso escrita (exclusivo)         | `&mut s`                 |
| Drop              | Liberação automática               | Fim de escopo            |

---

### ⚠️ **Dicas e Erros Comuns**

- **Clone para Copiar:** Se quiser copiar sem move, use `.clone()` (ex: `let s2 = s1.clone();`). Custa performance.  
- **Copy Trait:** Tipos como `i32`, `bool` implementam `Copy` — são copiados automaticamente.  
- **Stack vs Heap:** Moves em stack são baratos (cópia), mas em heap envolvem ponteiros (sem cópia de dados).  
- **Erro Comum:** "Borrow of moved value" — acontece quando você move e tenta usar depois. Solução: Use borrows ou clone.
