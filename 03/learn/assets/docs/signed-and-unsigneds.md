Em Rust, os números inteiros podem ser classificados como **assinados** (signed) ou **não assinados** (unsigned). Vou explicar de forma simples e clara, com exemplos.

### Números Assinados (Signed Integers)
- São tipos de inteiros que podem representar valores **positivos, negativos ou zero**.
- Eles usam a representação em **complemento de dois** (two's complement) para lidar com números negativos.
- Exemplos de tipos em Rust: `i8`, `i16`, `i32`, `i64`, `i128` (o "i" vem de "integer", e o número indica o tamanho em bits).
  - `i8`: Varia de -128 a 127 (8 bits).
  - `i32`: Varia de -2.147.483.648 a 2.147.483.647 (32 bits, tipo padrão para inteiros assinados).
- Uso: Ideal para cenários onde valores negativos são possíveis, como temperaturas abaixo de zero ou cálculos que envolvem subtrações que podem resultar em negativos.
- Exemplo de código:
  ```rust
  let x: i32 = -42;  // Valor negativo é permitido
  println!("{}", x);  // Saída: -42
  ```

### Números Não Assinados (Unsigned Integers)
- São tipos de inteiros que representam apenas valores **não negativos** (zero ou positivos).
- Não há representação para números negativos, o que permite um intervalo maior de valores positivos em comparação com o equivalente assinado.
- Exemplos de tipos em Rust: `u8`, `u16`, `u32`, `u64`, `u128` (o "u" vem de "unsigned").
  - `u8`: Varia de 0 a 255 (8 bits, útil para bytes ou valores pequenos).
  - `u32`: Varia de 0 a 4.294.967.295 (32 bits, tipo comum para contagens ou índices).
- Uso: Perfeito para quantidades que nunca são negativas, como tamanhos de arrays, contagens ou endereços de memória.
- Exemplo de código:
  ```rust
  let y: u32 = 42;  // Apenas positivo ou zero
  // let z: u32 = -1;  // Isso causa erro de compilação!
  println!("{}", y);  // Saída: 42
  ```

### Diferenças Principais
- **Intervalo de Valores**: Para o mesmo tamanho em bits, os não assinados têm o dobro do alcance positivo (ex: `i8` vai até 127, mas `u8` até 255).
- **Comportamento em Overflow**: Em Rust, por padrão (em modo debug), operações que causam overflow (ex: somar além do máximo) causam pânico. Em modo release, há wrap-around, mas é melhor usar métodos como `checked_add` para segurança.
- **Escolha do Tipo**: Use assinados quando negativos são possíveis; use não assinados para otimizar espaço e evitar erros com valores negativos inesperados.
- **Tipo Padrão**: Se você não especificar, Rust infere `i32` para literais inteiros (ex: `let a = 10;` é `i32`).

Se você tentar atribuir um valor negativo a um tipo não assinado, o compilador Rust vai reclamar na hora da compilação, o que ajuda a evitar bugs.
