use std::io;
use std::thread::sleep;
use std::time::Duration;

use rand::Rng;

struct Personagem {
    nome: String,
    vida: i32,
    max_vida: i32,
    nivel: i32,
    forca: i32,
    defesa: i32,
    defesa_ativa: i32,
    habilidade_usos: i32,
    turnos_pulados: i32,
    pocoes: i32,
}

impl Personagem {
    fn novo_guerreiro(nome: &str) -> Self {
        Self {
            nome: nome.to_string(),
            vida: 100,
            max_vida: 100,
            nivel: 1,
            forca: 12,
            defesa: 6,
            defesa_ativa: 0,
            habilidade_usos: 2,
            turnos_pulados: 0,
            pocoes: 2,
        }
    }

    fn novo_mago(nome: &str) -> Self {
        Self {
            nome: nome.to_string(),
            vida: 80,
            max_vida: 80,
            nivel: 1,
            forca: 18,
            defesa: 3,
            defesa_ativa: 0,
            habilidade_usos: 2,
            turnos_pulados: 0,
            pocoes: 2,
        }
    }

    fn mostrar_info(&self) {
        println!("Nome: {}", self.nome);
        println!("Vida: {} / {}", self.vida, self.max_vida);
        println!("Nível: {}", self.nivel);
        println!("Força: {}", self.forca);
        println!("Defesa: {}", self.defesa);
        println!("Habilidade especial restante: {}", self.habilidade_usos);
        println!("Poções restantes: {}", self.pocoes);
    }

    fn obter_vida(&self) -> i32 {
        self.vida
    }

    fn receber_dano(&mut self, dano: i32) {
        let dano_real = if self.defesa_ativa > 0 {
            let defesa_total = self.defesa + self.defesa_ativa;
            if dano > defesa_total {
                dano - defesa_total
            } else {
                1
            }
        } else if dano > self.defesa {
            dano - self.defesa
        } else {
            1
        };

        self.defesa_ativa = 0;
        self.vida -= dano_real;

        if self.vida < 0 {
            self.vida = 0;
        }
    }

    fn receber_dano_ignora_defesa(&mut self, dano: i32) {
        self.vida -= dano;

        if self.vida < 0 {
            self.vida = 0;
        }
    }

    fn recuperar_vida(&mut self, valor: i32) {
        self.vida += valor;
        if self.vida > self.max_vida {
            self.vida = self.max_vida;
        }
    }

    fn usar_pocao(&mut self) -> bool {
        if self.pocoes <= 0 {
            println!("{} não tem mais poções.", self.nome);
            return false;
        }

        if self.vida >= self.max_vida {
            println!("{} já está com a vida cheia.", self.nome);
            return false;
        }

        self.pocoes -= 1;
        let cura = 20;
        self.recuperar_vida(cura);
        println!("{} usou uma poção e recuperou {} de vida.", self.nome, cura);
        true
    }

    fn atacar(&self, alvo: &mut Personagem) {
        let dano = self.forca;
        println!("{} atacou {} e causou {} de dano.", self.nome, alvo.nome, dano);
        alvo.receber_dano(dano);
    }

    fn habilidade_especial(&mut self, alvo: &mut Personagem) -> bool {
        if self.habilidade_usos <= 0 {
            println!("{} não pode usar mais a habilidade especial.", self.nome);
            return false;
        }

        self.habilidade_usos -= 1;
        self.turnos_pulados += 2;

        let dano = self.forca + 5;
        println!(
            "{} usou habilidade especial! Causou {} de dano e perde 2 turnos.",
            self.nome, dano
        );
        alvo.receber_dano_ignora_defesa(dano);
        true
    }

    fn defender(&mut self) {
        self.defesa_ativa = 5;
        println!(
            "{} levantou a defesa e reduz o dano do próximo ataque em 5 pontos.",
            self.nome
        );
    }

    fn tem_turno_pulado(&mut self) -> bool {
        if self.turnos_pulados > 0 {
            self.turnos_pulados -= 1;
            println!("{} perdeu um turno por usar a habilidade especial.", self.nome);
            true
        } else {
            false
        }
    }

    fn acao_aleatoria(&mut self, alvo: &mut Personagem) {
        let mut rng = rand::thread_rng();
        let acao = rng.gen_range(1..=4);

        match acao {
            1 => self.atacar(alvo),
            2 => {
                if self.habilidade_usos > 0 {
                    self.habilidade_especial(alvo);
                } else {
                    self.atacar(alvo);
                }
            }
            3 => self.defender(),
            4 => {
                if self.pocoes > 0 {
                    self.usar_pocao();
                } else {
                    self.atacar(alvo);
                }
            }
            _ => self.atacar(alvo),
        }
    }
}

fn mostrar_menu() {
    println!("INITIAL FANTASY");
    println!("Para prosseguir, escolha uma opção:");
    println!("1 - Jogar");
    println!("2 - Instruções");
    println!("3 - Sobre o desenvolvedor");
    println!("0 - Sair");
}

fn ler_opcao() -> i32 {
    let mut entrada = String::new();
    io::stdin()
        .read_line(&mut entrada)
        .expect("Falha ao ler entrada");

    match entrada.trim().parse() {
        Ok(numero) => numero,
        Err(_) => {
            println!("Digite um número válido.");
            -1
        }
    }
}

fn mostrar_instrucoes() {
    loop {
        println!("\nINSTRUÇÕES");
        println!("1 - Como jogar");
        println!("2 - Regras de combate");
        println!("0 - Voltar");

        let opcao = ler_opcao();

        match opcao {
            1 => {
                println!("\nComo jogar:");
                println!("- Escolha uma ação do seu turno.");
                println!("- Ataque normal causa dano básico.");
                println!("- Habilidade especial causa mais dano, ignora defesa e perde 2 turnos.");
                println!("- Defender reduz o dano do próximo ataque.");
                println!("- Poções curam 20 de vida e cada personagem tem 2 por partida.");
                println!("- Ganha quem zerar a vida do oponente.");
                sleep(Duration::from_secs(3));
            }
            2 => {
                println!("\nRegras de combate:");
                println!("- Defesa ativa reduz dano do próximo ataque.");
                println!("- Habilidade especial tem uso limitado e custa 2 turnos de pausa.");
                println!("- O ataque especial vale a pena, mas exige custo.");
                sleep(Duration::from_secs(3));
            }
            0 => break,
            _ => {
                println!("Opção inválida. Tente novamente.");
                sleep(Duration::from_secs(3));
            }
        }
    }
}

fn escolher_personagem() -> (Personagem, Personagem) {
    loop {
        println!("\nEscolha sua classe:");
        println!("1 - Guerreiro");
        println!("2 - Mago");

        let escolha = ler_opcao();

        match escolha {
            1 => {
                let jogador = Personagem::novo_guerreiro("Jogador");
                let inimigo = Personagem::novo_mago("Inimigo");
                return (jogador, inimigo);
            }
            2 => {
                let jogador = Personagem::novo_mago("Jogador");
                let inimigo = Personagem::novo_guerreiro("Inimigo");
                return (jogador, inimigo);
            }
            _ => {
                println!("Escolha inválida. Tente novamente.");
                sleep(Duration::from_secs(3));
            }
        }
    }
}

fn iniciar_jogo() {
    println!("\nIniciando a batalha...\n");
    sleep(Duration::from_secs(3));

    let (mut jogador, mut inimigo) = escolher_personagem();

    println!("\nSeu personagem:");
    jogador.mostrar_info();
    sleep(Duration::from_secs(3));

    println!("\nSeu adversário:");
    inimigo.mostrar_info();
    sleep(Duration::from_secs(3));

    loop {
        if jogador.obter_vida() <= 0 {
            println!("\n{} venceu a batalha!", inimigo.nome);
            break;
        }

        if inimigo.obter_vida() <= 0 {
            println!("\n{} venceu a batalha!", jogador.nome);
            break;
        }

        if jogador.turnos_pulados > 0 {
            if jogador.tem_turno_pulado() {
                sleep(Duration::from_secs(3));
                continue;
            }
        }

        println!("\nSua vez, {}!", jogador.nome);
        println!("Escolha sua ação:");
        println!("1 - Ataque normal");
        println!("2 - Habilidade especial");
        println!("3 - Defender");
        println!("4 - Usar poção");

        let escolha = ler_opcao();

        match escolha {
            1 => jogador.atacar(&mut inimigo),
            2 => {
                if !jogador.habilidade_especial(&mut inimigo) {
                    println!("A habilidade especial não pode ser usada agora.");
                    sleep(Duration::from_secs(3));
                    continue;
                }
            }
            3 => jogador.defender(),
            4 => {
                jogador.usar_pocao();
            }
            _ => {
                println!("Ação inválida. O turno foi perdido.");
                sleep(Duration::from_secs(3));
                continue;
            }
        }

        println!("Vida restante do {}: {}", inimigo.nome, inimigo.obter_vida());

        if inimigo.obter_vida() <= 0 {
            println!("\n{} venceu a batalha!", jogador.nome);
            break;
        }

        sleep(Duration::from_secs(3));

        if inimigo.turnos_pulados > 0 {
            if inimigo.tem_turno_pulado() {
                sleep(Duration::from_secs(3));
                continue;
            }
        }

        println!("\nTurno do {}!", inimigo.nome);
        inimigo.acao_aleatoria(&mut jogador);
        println!("Vida restante do {}: {}", jogador.nome, jogador.obter_vida());

        if jogador.obter_vida() <= 0 {
            println!("\n{} venceu a batalha!", inimigo.nome);
            break;
        }

        sleep(Duration::from_secs(3));
    }
}

fn mostrar_sobre() {
    println!("\nDesenvolvido por: oAndersus404");
    println!("Projeto: Initial Fantasy\n");
}

fn main() {
    loop {
        mostrar_menu();
        let escolha = ler_opcao();

        match escolha {
            1 => {
                iniciar_jogo();
                sleep(Duration::from_secs(3));
            }
            2 => mostrar_instrucoes(),
            3 => {
                mostrar_sobre();
                sleep(Duration::from_secs(3));
            }
            0 => {
                println!("Saindo...");
                break;
            }
            _ => {
                println!("Opção inválida, tente novamente.");
                sleep(Duration::from_secs(3));
            }
        }
    }
}
