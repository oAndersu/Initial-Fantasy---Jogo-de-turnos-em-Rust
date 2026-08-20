# ⚔️ Initial Fantasy

Um RPG de batalha por turnos desenvolvido em **Rust**, executado diretamente no terminal.

## 📖 Sobre o projeto

**Initial Fantasy** é um projeto desenvolvido para praticar conceitos da linguagem Rust através da criação de um pequeno RPG de combate.

O jogador escolhe uma classe e enfrenta um inimigo controlado pelo computador em uma batalha por turnos.

Atualmente existem duas classes:

* ⚔️ **Guerreiro**
* 🧙 **Mago**

Cada personagem possui atributos próprios de vida, força e defesa.

## 🎮 Sistema de combate

Durante o seu turno, o jogador pode escolher entre quatro ações:

1. **Ataque normal** — causa dano baseado na força do personagem.
2. **Habilidade especial** — causa dano adicional, ignora a defesa do adversário e possui usos limitados.
3. **Defender** — aumenta temporariamente a defesa contra o próximo ataque.
4. **Usar poção** — recupera parte da vida do personagem.

A habilidade especial possui um custo adicional: depois de utilizá-la, o personagem perde dois turnos.

O inimigo possui uma IA simples que escolhe aleatoriamente entre atacar, utilizar a habilidade especial, defender ou usar uma poção.

## 🧙 Classes

### ⚔️ Guerreiro

* Vida: **100**
* Força: **12**
* Defesa: **6**
* Poções: **2**
* Habilidades especiais: **2**

### 🧙 Mago

* Vida: **80**
* Força: **18**
* Defesa: **3**
* Poções: **2**
* Habilidades especiais: **2**

Os valores são definidos diretamente na criação de cada personagem.

## 🛠️ Tecnologias

* **Rust**
* **Cargo**
* **rand**

O projeto utiliza a biblioteca `rand` para gerar as ações aleatórias do inimigo.

## ▶️ Como executar

É necessário ter o **Rust e o Cargo** instalados.

Clone o repositório:

```bash
git clone https://github.com/oAndersu/Initial-Fantasy---Jogo-de-turnos-em-Rust
cd initial-fantasy
```

Execute o projeto:

```bash
cargo run
```

Para gerar uma versão compilada:

```bash
cargo build --release
```

## 📂 Estrutura atual

```text
initial-fantasy/
├── src/
│   └── main.rs
├── Cargo.toml
├── Cargo.lock
└── README.md
```

Atualmente, a maior parte da lógica do jogo está concentrada em `main.rs`, incluindo personagens, combate, menus e fluxo principal.

## 🚧 Próximos objetivos

O projeto ainda está em desenvolvimento. Algumas possibilidades para futuras versões:

* [ ] Adicionar novas classes
* [ ] Criar mais inimigos
* [ ] Adicionar diferentes habilidades
* [ ] Criar sistema de experiência e níveis
* [ ] Melhorar a IA dos inimigos
* [ ] Criar sistema de fases
* [ ] Adicionar equipamentos
* [ ] Separar o código em diferentes módulos
* [ ] Melhorar a interface do terminal
* [ ] Adicionar sistema de salvamento

## 🎓 Objetivo

O principal objetivo do **Initial Fantasy** é servir como projeto de aprendizado e prática de programação em Rust, evoluindo gradualmente conforme novos conceitos da linguagem são estudados.

## 👨‍💻 Desenvolvedores

**oAndersus404, Natanglx, Moskitao, Luan, Gustavo**

Projeto desenvolvido como parte do processo de aprendizado de **Rust**.
