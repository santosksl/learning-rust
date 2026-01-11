**Stack e Heap em Rust**

Em Rust (e na maioria das linguagens de programação), **stack** (pilha) e **heap** são duas regiões diferentes da memória usadas para armazenar dados. A principal diferença está em **como** e **quando** a memória é alocada e liberada.

---

### 📚 **Stack (Pilha)**

- **O que é?**  
  Região de memória de **acesso muito rápido**, organizada como uma pilha (LIFO — Last In, First Out).  
  Usada para variáveis **de tamanho fixo** e **conhecido em tempo de compilação**.

- **Características principais:**
  - Alocação **automática** quando a variável é declarada
  - Desalocação **automática** quando a variável sai de escopo (fim do bloco `{}`)
  - **Muito rápido** (acesso O(1))
  - Tamanho **limitado** (geralmente alguns MB — depende do sistema)
  - Dados são **contíguos** na memória

- **Exemplo em Rust:**
```rust
fn main() {
    let x = 42;                    // → alocado na stack
    let s = "hello";               // &str → dados na stack
    let arr = [1, 2, 3, 4];        // array fixo → stack

    println!("x = {}", x);
} // x, s e arr são automaticamente desalocados aqui
```

**Quando usar stack?**  
Primitivos (`i32`, `f64`, `bool`), tuplas pequenas, arrays fixos, structs pequenas, referências (`&T`).

---

### 🗃️ **Heap (Monte)**

- **O que é?**  
  Região de memória **dinâmica**, usada quando o tamanho dos dados **não é conhecido em tempo de compilação**.

- **Características principais:**
  - Alocação **em tempo de execução** (runtime)
  - **Mais lenta** que a stack (alocação + desalocação)
  - Tamanho **praticamente ilimitado** (limitado pela RAM disponível)
  - Dados **não são contíguos** (fragmentação possível)
  - Gerenciada via **ownership** do Rust → sem garbage collector!

- **Como alocar no heap em Rust?**  
  Use smart pointers como `Box`, `Vec`, `String`, `Rc`, `Arc`, etc.

- **Exemplo em Rust:**
```rust
fn main() {
    let v: Vec<i32> = vec![1, 2, 3, 4, 5]; // → dados no heap
    let b = Box::new(42);                  // → 42 alocado no heap

    println!("{:?}", v);
} // Vec e Box são desalocados automaticamente aqui (drop)
```

**Quando usar heap?**
- Strings dinâmicas (`String`)
- Vetores dinâmicos (`Vec<T>`)
- Estruturas grandes ou de tamanho variável
- Dados que precisam viver além do escopo atual

---

### 🔄 **Diferenças principais**

| Aspecto              | Stack                          | Heap                              |
|----------------------|--------------------------------|-----------------------------------|
| Alocação             | Automática, em tempo de compilação | Manual (via Box/Vec/etc) em runtime |
| Velocidade           | Muito rápida                   | Mais lenta                        |
| Tamanho              | Limitado (ex: 1–8 MB)          | Quase ilimitado                   |
| Desalocação          | Automática (fim de escopo)     | Automática via ownership/drop     |
| Acesso              | Direto (índice)                | Indireto (ponteiro)               |
| Dados                | Fixos, pequenos                | Dinâmicos, grandes                |
| Exemplo              | `let x = 5;`                   | `let v = vec![1,2,3];`            |

---

### 🛡️ **Como o Rust gerencia isso com segurança?**

Rust usa o sistema de **ownership + borrowing** para garantir:
- Nenhum **memory leak** (memória é liberada automaticamente via `Drop`)
- Nenhum **use-after-free**
- Nenhum **data race** em multithreading
- Sem necessidade de **garbage collector** (diferente de Java/Go/Python)

Exemplo de transferência de ownership:
```rust
fn foo(s: String) { // ownership é movido para foo
    println!("{}", s);
}

fn main() {
    let s = String::from("hello"); // alocado no heap
    foo(s); // s é movido → não pode mais ser usado aqui
    // println!("{}", s); // erro de compilação!
}
```

---

### ⚠️ **Dica importante**

- **Stack overflow** → acontece quando a pilha fica cheia (ex: recursão profunda)
- Rust **não permite** alocação dinâmica na stack (sem `alloca` como em C)

---

**Resumo simples:**
- **Stack** = rápido, automático, tamanho fixo → use para dados pequenos e conhecidos
- **Heap** = dinâmico, via `Box`/`Vec`/`String` → use quando o tamanho varia em runtime
