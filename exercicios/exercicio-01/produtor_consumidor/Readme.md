# Exercício 01 — Produtor e Consumidor em Rust

Aplicação em Rust que implementa o clássico problema **produtor–consumidor** utilizando duas threads que compartilham um buffer limitado.

O acesso ao buffer é protegido por exclusão mútua com `Mutex`, e a coordenação entre as threads é feita com variáveis de condição (`Condvar`).

O código-fonte está localizado na pasta `produtor_consumidor/`.

## Objetivo

Implementar um processo com:

- Uma thread **produtora**, responsável por inserir itens no buffer.
- Uma thread **consumidora**, responsável por remover itens do buffer.
- Um **buffer limitado**, com capacidade máxima fixa.
- **Exclusão mútua** no acesso ao buffer compartilhado.

## Funcionamento

O produtor gera números inteiros sequenciais, de `1` a `12`, e insere cada um no buffer.

O consumidor remove os itens na mesma ordem em que foram inseridos, comportando-se como uma fila FIFO.

O buffer possui capacidade para **5 itens**. Portanto:

- Se o buffer estiver **cheio**, o produtor aguarda até que o consumidor remova um item.
- Se o buffer estiver **vazio**, o consumidor aguarda até que o produtor insira um item.

## Sincronização

A aplicação utiliza os seguintes recursos da biblioteca padrão de Rust:

| Recurso | Função |
|---|---|
| `Mutex` | Garante exclusão mútua no acesso ao buffer compartilhado |
| `Condvar` | Permite que uma thread aguarde até que uma condição seja satisfeita |
| `Arc` | Permite compartilhar o mesmo estado entre as threads |
| `thread::spawn` | Cria as threads produtora e consumidora |

Foram utilizadas duas variáveis de condição:

- `not_full`: usada pelo produtor para aguardar espaço disponível no buffer.
- `not_empty`: usada pelo consumidor para aguardar a existência de itens no buffer.

O uso de `while` antes de cada `wait()` garante que a condição seja reavaliada após a thread acordar, evitando problemas com despertares espúrios.

## Estrutura do projeto

```text
exercicio-01/
├── README.md
└── produtor_consumidor/
    ├── Cargo.toml
    ├── saida_codigo.png
    └── src/
        └── main.rs
```

## Como compilar e executar

É necessário ter o Rust instalado. Caso não tenha, instale com:

```bash
curl --proto '=https' --tlsv1.2 -sSf [https://sh.rustup.rs](https://sh.rustup.rs) | sh
```

Entre na pasta do projeto:

```bash
cd produtor_consumidor
```

Execute a aplicação:

```bash
cargo run
```

## Saída do programa

A imagem abaixo mostra a saída gerada pela execução do programa:

![Saída do código](produtor_consumidor/saida_codigo.png)

A ordem exata das mensagens pode variar entre execuções, pois as threads são escalonadas pelo sistema operacional. Entretanto, o buffer nunca ultrapassa a capacidade máxima de 5 elementos.

## Conceitos aplicados

- Programação concorrente com threads.
- Problema clássico do produtor–consumidor.
- Buffer limitado.
- Exclusão mútua.
- Variáveis de condição.
- Sincronização entre threads.
- Segurança de memória garantida pelo sistema de tipos de Rust.
