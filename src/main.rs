/*! 
    Alterna wallpapers, ou "exibição de wallpapers" durante tempos e tempos,
 contando com meses com comemorações específicas como o natal e halloween. 
 Ele aceita uma expansão dinâmica dos wallapapers que serão alternados, 
 apenas coloque mais nos diretórios específicos que ele lê. Será computado 
 em toda inicialização do computador, ou novo login.
*/

// Biblioteca externas:
#[allow(warnings)]
extern crate utilitarios;
use utilitarios::aleatorio::sortear;
use utilitarios::legivel::tempo as tempo_l;
// Bibliotecas do Rust:
use std::time::{Instant, Duration};
use std::thread::sleep;
// Módulos do próprio projeto:
mod banco_de_dados;
mod transicao;
mod atualizacoes;
mod comparacao;
mod compilacao;
mod notificacoes;
mod configuracao;
mod constantes;

// use temporizador::*;
use transicao::{
   alterna_transicao,
   duracao_atual_transicao
};
use atualizacoes::atualiza_xmls;
use constantes::*;

fn pausa_aleatoria() {
   /* pausa de alguns minutos para se curtir a transição anterior. 
    * Claro, eles estão quantificados em segundos. */
   let minutos = sortear::u64(10*60..=15*60);
   let limite = Duration::from_secs(minutos);
   let timer = Instant::now();

   while timer.elapsed() < limite {
      let restante = limite - timer.elapsed();

      println!(
         "Tempo de espera para iniciar de fato ...{:>10}", 
         tempo_l(restante.as_secs(), true)
      );
      sleep(Duration::from_secs(sortear::u64(9..=43)));
   }
}

use std::env::var;
use std::path::{Path};

/** Verificação e configuração de variáveis, diretórios, e etc. */
fn checagem_e_configuracao_do_ambiente() {
   let inputs = ["RUST_CODES", "XDG_CURRENT_DESKTOP"];

   println!("\nChecagem de variáveis de ambientes estão okay...");

   for variavel in inputs {
      assert!(var(variavel).is_ok()); 
      println!("Variável '{variavel:}' presente.");
   }

   let mut inputs = vec![
       PYTHON, ARQUIVO_CONF, ULTIMA_NOTIFICACAO, 
       CAMINHO_ARQUIVO, BD1
   ];
   println!("Verificação de todos caminhos do código ...");

   for caminho_str in inputs.drain(..) {
      let caminho = Path::new(caminho_str);

      print!("{caminho_str:} ... ");
      if caminho.exists()
         { println!("\u{10003}"); }
      else
         { println!("\u{10007}"); }
   }

}

fn main() {
   checagem_e_configuracao_do_ambiente();
   /* se for o artefato de depuração, então já colocar em caminho uma 
    * possível compilação da versão otimizada. */
    if compilacao::executa_compilacao()
      { println!("Uma compilação foi realizada!"); }
   else
      { println!("Nenhuma compilação foi inicializada!"); }

   /* Toda vez que for acionada no começo de do sistema/ou login uma 
    * nova 'transição de wallpapers' será escolhidas baseado na data de 
    * acionamento. Agora, o programa também escolha uma nova 'transição',
    * sem precisar nova inicialização do sistema/ou login;... em média, 
    * de 5 à 8 horas, decorrida da última mudança.  */
   if !cfg!(debug_assertions)
      { pausa_aleatoria();}
   else
      { println!("Pausa instantânea!"); }

   // marcando tempo inicial de contagem ...
   let mut cronometro = Instant::now();
   /* selecionando um 'tempo final' para que ao passar tal, aciona uma 
    * nova transição-de-walpapers. Tal tempo 'tf' estará entre 
    * um MÁXIMO E MÍNIMO. */
   let tf:u16 = sortear::u16(MINIMO..=MAXIMO);
   let mut tempo_final = Duration::from_secs(tf as u64);
   let mut execucao_inicial = false;

   loop {
      let aciona_uma_nova_transicao = {
      /* Se tiver "atigindo" tal tempo, então trocar a transição de 
       wallpaper atual. */
         cronometro.elapsed() > tempo_final
                     &&
         execucao_inicial
      };

      if aciona_uma_nova_transicao {
         alterna_transicao();
         // Zerá contador... para nova contagem.
         cronometro = Instant::now();
         /* pegando duração do tempo total de apresentação da nova 
          * transição de slides, somado à 1min para acabar toda 
          * apresentação. */
         tempo_final = duracao_atual_transicao() + Duration::from_secs(60);
         // mostra a notificação da atual ação.
         notificacoes::popup_notificacao_de_transicao();

      } else if !execucao_inicial {
         // faz uma execução inicial
         alterna_transicao();
         // mostra a notificação da atual ação.
         notificacoes::popup_notificacao_de_transicao();
         // executada uma vez ...
         execucao_inicial = true;

      } else {
         // obtendo tempo para próxima transição.
         let tempo_restante = tempo_final - cronometro.elapsed();
         // traduzindo segundos para algo legível.
         println!(
            "contagem regressiva para próxima transição {}",
            tempo_l(tempo_restante.as_secs(), true)
         );
      }

      /* Possívelmente realizando uma atualização dos arquivlos de 
       * transições XMLs no diretório onde ficam. */
      if false { atualiza_xmls(); } // desabilitado por enquanto

      // intercalar os loops à cada, aproximadamente, 1 min.
      sleep(Duration::from_secs(sortear::u64(30..=60)));
   }
}

