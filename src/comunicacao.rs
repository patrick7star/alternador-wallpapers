extern crate libc;

use std::ffi::{CString, c_void as void};
use std::io;
use std::mem::{transmute};
use std::time::{Duration, Instant};
use std::thread::{spawn as start_new_thread, sleep, JoinHandle};
use std::path::{PathBuf};
use crate::linque::{computa_caminho};


pub fn transmissor(cliente: JoinHandle<()>) -> JoinHandle<()>
{
/* Emite sinais, via um 'named pipe', pra algum transmissor em outro
 * processo. Também, lança uma thread pro chamador. Antes deste ser chamado,
 * é preciso esperar qualquer outra thread 'receptor' lançada no mesmo
 * processo. */
   cliente.join().expect
      ("Preciso parar a antena, antes de começar o próprio transmissor.");

   let mut ritmo = Duration::from_millis(300);
   let mut contador = 0;
   let mut relogio = Instant::now();

   start_new_thread(|| { loop {
      let _= alterna_interruptor(true);
   }});

   start_new_thread(move ||{
      loop {
         sleep(ritmo);
         info_da_transmissao(&mut contador, ritmo, &mut relogio);
         ritmo = funcao_tempo(contador, ritmo);
      }
   })
}

pub fn receptor() -> JoinHandle<()> {
/* Lança uma thread cliente, onde ela tenta por algum instante receptar
 * alguma sinal, ou mesmo erro ocorrido de outro processo. Tal valor que
 * foi enviado via 'named pipe'. */
   let mensagem_de_sucesso = "Todas tentativas indicam que não há um \
         programa rodando no momento.";
   let mensagem_de_erro = "Já existe um programa rodando!";

   cria_tubulacao().unwrap();
   std::thread::spawn(move || {
      match atual_estado() {
         Ok(false)=>
            { println!("\n{}", mensagem_de_sucesso); }
         Ok(true) =>
            { println!("{}", mensagem_de_erro); std::process::exit(2); }
         Err(erro) =>
            { panic!("[erro] {}", erro.kind()); }
      }
   })
}


fn caminho_da_tubulacao() -> PathBuf {
// Caminho do 'named pipe' tratado neste módulo.
   const ARQUIVO: &str = "./ativo";
   let output = computa_caminho(ARQUIVO);

   output
}

fn caminho_da_tubulacao_cstr() -> CString {
// O mesmo que o acima, porém transforma o caminho em algo compatível com C.
   let caminho_pth = caminho_da_tubulacao();
   let caminho_str = caminho_pth.to_str().unwrap();
   let caminho = CString::new(caminho_str);

   caminho.unwrap()
}

fn cria_tubulacao() -> io::Result<()> {
/* Cria a via, onde a informação da execução será transmitida. */
   let caminho = caminho_da_tubulacao_cstr();
   let pathname = caminho.clone().into_raw();
   let permissoes = 0o600;

   unsafe {
      if libc::mkfifo(pathname, permissoes) == 0
         { println!("Named pipe {:?} criado com sucesso.", caminho); }
      else {
         match *libc::__errno_location() {
            libc::EEXIST =>
               { println!("Tubulação já existe prá isso."); }
            _=>
               { assert!(false, "Caso ainda não trabalhado!"); }
         }
      }
   }

   Ok(())
}

fn alterna_interruptor(estado: bool) -> io::Result<()> {
/* Envia dados informando que este programa está rodando. O tempo que ele
 * mantém enviando é meio à um segundo e meio, então fecha a linha. */
   let caminho = caminho_da_tubulacao_cstr();
   let pathname = caminho.clone().into_raw();
   let valor = i8::from(estado);
   let via: i32;
   let reinterpret_cast = transmute::<&i8, *const i8>;
   let bytes = unsafe { reinterpret_cast(&valor) };
   const ESPERA: Duration = Duration::from_millis(500);

   unsafe {
   /* Tal transmissor de informação pra outros processos, faz o seguinte:
    * abre o 'caminho(named pipe)', escreve bytes(envia), então aguarda por
    * meio segundo pra ver se alguém ler; com leitura ou não, fecha o
    * 'named pipe' aberto. */
      via = libc::open(pathname, libc::O_RDWR);
      libc::write(via, bytes as *const void, 1);
      sleep(ESPERA);
      libc::close(via);
   }
   Ok(())
}

fn atual_estado() -> io::Result<bool> {
/* Tenta ler algum dado por um instante de tempo. */
   let caminho = caminho_da_tubulacao_cstr();
   let pathname = caminho.clone().into_raw();
   let mut valor: i8 = i8::MAX / 2;
   let reinterpret_cast = transmute::<&mut i8, *mut i8>;
   let pointer = unsafe { reinterpret_cast(&mut valor) };
   let relogio = Instant::now();
   const RITMO: Duration = Duration::from_millis(150);

   unsafe {
      let modo = libc::O_RDWR | libc::O_NONBLOCK;
      let via: i32 = libc::open(pathname, modo);

      while relogio.elapsed() < (6 * RITMO) {
         libc::read(via, pointer as *mut void, 1);
         std::thread::sleep(RITMO);
      }
      libc::close(via);
   }
   Ok(valor == 1)
}

fn info_da_transmissao(disparos: &mut i32, ritmo: Duration,
  relogio: &mut Instant)
{
   const MOSTRAR_NOVAMENTE: Duration = Duration::from_secs(3);

   if relogio.elapsed() > MOSTRAR_NOVAMENTE {
      println!("Transmitido pela {:>4}ª vez.\t~{:0.0?}", *disparos, ritmo);

      *disparos += 1;
      *relogio = Instant::now();
   }
}

fn funcao_tempo(disparos: i32, ritmo: Duration) -> Duration {
/* Função do 'ritmo' ao longo dos disparos e tempo definido atualmente. */
   const UM_MINUTO: Duration = Duration::from_secs(60);
   const TOTAL: i32 = 5;
   const TAXA: f32 = 1.12; // 12%

   if disparos % TOTAL == 0 {
      if ritmo > UM_MINUTO
         { ritmo }
      else
         { Duration::mul_f32(ritmo, TAXA) } }
   else
      { ritmo }
}


#[allow(non_snake_case, unused_imports)]
mod tests {
   use std::sync::{Arc, Mutex};
   use std::cell::{Cell};
   use super::{
      start_new_thread, libc, CString, caminho_da_tubulacao_cstr,
      caminho_da_tubulacao
   };

   #[test]
   fn uma_instancia_do_programa_ja_aberta() {
      let contagem = Arc::new(Mutex::new(Cell::new(0)));
      let count = Arc::clone(&contagem);

      super::cria_tubulacao().unwrap();

      let fio_a = start_new_thread(move || {
         super::alterna_interruptor(true).unwrap();
      });
      let fio_b = start_new_thread(move || {
         if let Ok(_) = super::atual_estado() {
            if let Ok(valor) = count.lock() {
               let x = valor.get();

               valor.set(x + 1);
               println!("Dados recebido com sucesso.");
            }
         }

         let valor = (*count).lock().unwrap().get();
         println!("{:?} leituras realizadas com sucesso.", valor);
      });

      for fio in [fio_a, fio_b]
         { fio.join().unwrap(); }

      // Como aqui, neste estágio, já não haverá mais concorrência de dados
      // é normal aplicar o 'unwrap'.
      assert!((*contagem).lock().unwrap().get() > 0);
      println!("Tudo foi terminado com sucesso.");
   }

   #[test]
   fn tamanho_do_buffer() { unsafe {
      let caminho = caminho_da_tubulacao_cstr();
      let caminho = caminho.into_raw();
      let fd = libc::open(caminho, libc::O_RDWR);
      let mut capacidade = libc::fcntl(fd, libc::F_GETPIPE_SZ);
      let mut num_de_paginas = capacidade / (4 * 1024);

      println!(
         "Tamanho do buffer: {}\nNúmero de páginas: {}",
         capacidade, num_de_paginas
      );

      libc::fcntl(fd, libc::F_SETPIPE_SZ, 4096);
      capacidade = libc::fcntl(fd, libc::F_GETPIPE_SZ);
      num_de_paginas = capacidade / (4 * 1024);

      println!(
         "\tTamanho do buffer: {}\n\tNúmero de páginas: {}",
         capacidade, num_de_paginas
      );
      libc::close(fd);
   }}

   #[test]
   fn verifica_caminho_da_tubulacao() {
      let output_a = caminho_da_tubulacao();
      let output_b = caminho_da_tubulacao_cstr();

      println!("{}", output_a.display());
      println!("CString interna: {:?}", output_b);
   }
}
