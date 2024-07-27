
# Comparação de Programação Multithread: Java vs Rust

Este repositório contém exemplos práticos de programação multithread em Java e Rust, com foco em comparar o desempenho e as abordagens de gerenciamento de threads e memória em ambas as linguagens. Estes exemplos fazem parte de um vídeo requisitado na cadeira de Introdução ao Processamento Paralelo e Distribuído, ministrada pelo professor Gerson.

## Estrutura do Repositório

- `examples/`: Contém os códigos sobre multithread em Rust, bem como a comparação dos códigos Java e Rust para programação multithread.

## Executando os Exemplos

### Rust

1. Navegue até o diretório `examples/`.
2. Compile e execute o exemplo de multithread em Rust:

```bash
cd examples
cargo run --example rust_multithread_example
```

### Java

1. Navegue até o diretório `java-rust/`.
2. Compile e execute o exemplo de multithread em Java:

```bash
cd java-rust
javac MultithreadExample.java
java MultithreadExample
```

## Conteúdo dos Diretórios

### examples/

Este diretório contém o exemplo de multithread em Rust. O código mostra como criar e gerenciar múltiplas threads, utilizando `Arc` e `Mutex` para compartilhar e sincronizar dados.

### java-rust/

Este diretório contém exemplos em Java e Rust para comparação. Inclui um README detalhado explicando as diferenças entre as abordagens de ambas as linguagens e comparando os tempos de execução.

## Comparação de Desempenho

### Java

```java
All tasks completed in: 520 ms.
```

### Rust

```rust
All tasks completed in: 1.001948862s
```

Embora o Rust tenha demorado mais tempo para concluir as tarefas neste exemplo específico, é importante considerar os fatores que influenciam o desempenho, como o overhead de gerenciamento de threads e sincronização de dados. Rust oferece um controle de baixo nível e otimizações que podem resultar em desempenho superior em outros cenários.

## Conclusão

A programação multithread pode ser complexa, mas Rust se destaca ao fornecer segurança de memória sem a necessidade de um garbage collector, resultando em código mais seguro e eficiente. Java, por outro lado, facilita a programação multithread com abstrações de alto nível como `ExecutorService`, mas depende do garbage collector para gerenciamento de memória.


### Recursos Multithreads em Rust

```markdown
# Exemplo de Programação Multithread em Rust

Estes contém exemplos de programação multithread em Rust, demonstrando como utilizar threads para realizar tarefas concorrentes e gerenciar dados compartilhados entre elas.

## Estrutura do Projeto

O projeto está estruturado da seguinte forma:

```
multithread_examples/
├── Cargo.toml
├── src/
│   └── main.rs
└── examples/
    ├── create_threads.rs
    ├── channels.rs
    └── arc_mutex.rs
    └── MultithreadExample.java
    └── rust_multithread_example.rs
```

- **Cargo.toml**: Arquivo de configuração do projeto Cargo.
- **src/main.rs**: O arquivo principal do projeto (não utilizado neste exemplo específico).
- **examples/**: Diretório contendo os exemplos de código para multithreading.

## Exemplos de Multithreading

### 1. Criação de Threads (`create_threads.rs`)

Este exemplo demonstra como criar e executar múltiplas threads em Rust.

**Código:**
```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hello from the spawned thread! {}", i);
        }
    });

    for i in 1..5 {
        println!("Hello from the main thread! {}", i);
    }

    handle.join().unwrap();
}
```

**Descrição:**
- Cria uma thread que imprime mensagens de "Hello" com números.
- A thread principal também imprime mensagens.
- Usa `handle.join()` para esperar que a thread secundária termine.

### 2. Comunicação entre Threads usando Canais (`channels.rs`)

Este exemplo mostra como usar canais para comunicar dados entre threads.

**Código:**
```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
```

**Descrição:**
- Cria um canal para comunicação entre threads.
- A thread secundária envia a string "hi" através do canal.
- A thread principal recebe e imprime a mensagem.

### 3. Compartilhamento de Dados com Arc e Mutex (`arc_mutex.rs`)

Este exemplo demonstra como usar `Arc` e `Mutex` para compartilhar dados entre múltiplas threads de forma segura.

**Código:**
```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

**Descrição:**
- Cria um contador compartilhado usando `Arc` e `Mutex`.
- 10 threads incrementam o contador.
- Usa `Arc` para compartilhar o `Mutex` entre as threads.
- Usa `Mutex` para garantir acesso exclusivo ao contador.

## Como Executar os Exemplos

Para compilar e executar cada exemplo, use o comando `cargo run --example` seguido pelo nome do arquivo do exemplo (sem a extensão `.rs`).

**Exemplos:**

- **Criação de Threads:**
  ```sh
  cargo run --example create_threads
  ```

- **Comunicação entre Threads usando Canais:**
  ```sh
  cargo run --example channels
  ```

- **Compartilhamento de Dados com Arc e Mutex:**
  ```sh
  cargo run --example arc_mutex
  ```

## Requisitos

- Rust (instale-o a partir de [rust-lang.org](https://www.rust-lang.org/))
- Cargo (gerenciador de pacotes do Rust)

## Contribuições

Sinta-se à vontade para enviar pull requests ou abrir issues para sugestões e melhorias.

## Licença

Este projeto é licenciado sob a [MIT License](LICENSE).

```

### Explicações Adicionais

- **Introdução:** Apresenta o projeto e o que ele contém.
- **Estrutura do Projeto:** Explica a organização dos arquivos no projeto.
- **Exemplos de Multithreading:** Fornece uma descrição e o código para cada exemplo.
- **Como Executar os Exemplos:** Instruções sobre como compilar e executar os exemplos.
- **Requisitos:** Lista o que é necessário para executar o projeto.
- **Contribuições e Licença:** Orientações sobre como contribuir e a licença do projeto.

Se precisar ajustar ou adicionar algo mais, me avise!
