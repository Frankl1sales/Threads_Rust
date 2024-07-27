
# Comparação de Programação Multithread: Java vs Rust

Este repositório contém exemplos práticos de programação multithread em Java e Rust, com foco em comparar o desempenho e as abordagens de gerenciamento de threads e memória em ambas as linguagens. Estes exemplos fazem parte de um vídeo requisitado na cadeira de Introdução ao Processamento Paralelo e Distribuído, ministrada pelo professor Gerson.

## Estrutura do Repositório

- `examples/`: Contém os códigos sobre multithread em Rust.
- `java-rust/`: Contém a comparação dos códigos Java e Rust para programação multithread.

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

## Contato

Para mais informações, sinta-se à vontade para entrar em contato:

<!--- **Email**: seu_email@exemplo.com
- **YouTube**: [Seu Canal no YouTube](https://www.youtube.com/seucanal)-->


Agradecemos por explorar este repositório e esperamos que os exemplos fornecidos sejam úteis para entender as diferenças entre Java e Rust na programação multithread.
