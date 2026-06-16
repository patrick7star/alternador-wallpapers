/**
  *   Novo alternador de wallpapers, porém este não contará com XMLs 
  * gerados, e sim contará com simplesmente o Rust para detectar arquivos de 
  * imagens e com ele próprio em alternar o tipo de visualização. Aqui 
  * também, ele não sorteará uma transição automaticamente, mas sim, baseado 
  * naquelas que não foram sorteadas ainda.

  *   Acho sim, que o antigo código ainda ficará aqui, porém acho ele muito
  * difícil de reler, e sei, já reescrevi-lo várias vezes, porém, não fica
  * bom já que é sempre a mesma estrutura. Este novo já terá toda uma nova
  * estrutura para transicionar os wallpapers.
  */

// Biblioteca padrão do Rust:
use std::ffi::{OsStr};
use std::path::{PathBuf, Path};
use std::time::{Duration, Instant};
use std::fmt::{Debug};
use std::process::{Command};
use std::env::{self};
use std::collections::{VecDeque, HashSet};

// Módulos e submódulos do projeto:
use super::configuracao::{raiz_wallpapers, wallpapers_externos};
// Bibliotecas externas:
use utilitarios::aleatorio::{sortear};
use utilitarios::legivel::{tempo_legivel_duration};

type FilaImagens = VecDeque<PathBuf>;
type PilhaCaminhos = Vec<PathBuf>;


/* Por enquanto, o mesmo algoritmo que o antigo. Entretanto, este apesar de
 * considerar o XML, o match para isso será substituido futuramente por
 * outro algoritmo. Continuando, aqui, ao invés dele retornar os caminhos
 * dos XMLs, ele retorna o caminho do diretório. */
fn todos_diretorios_de_wallpapers() -> Vec<PathBuf> 
{
   /* Pegando já "wallpapers externos" à raiz, porém filtrando apenas
    * caminhos existentes. */
   let mut arquivos_xml = {
      wallpapers_externos().drain(..)
      .filter(|path| path.exists())
      .collect::<Vec<PathBuf>>()
   };

   // varrendo tal raíz, e colhendo arquivos XML's...
   for dir in raiz_wallpapers().read_dir().unwrap() 
   {
      let entrada = dir.unwrap().path();
      let diretorio = entrada.as_path();

      /* Se for um diretório, entra nele e varre-o por um padrão, que 
       * é: verificar se há arquivos de imagens e um xml com o nome do 
       * diretório. O algoritmo, por enquanto, só entra um subdiretório
       * atrás de XML's, futuramente a busca será mais profunda. */
      if diretorio.is_dir() 
      {
         for subdir in diretorio.read_dir().unwrap() 
         {
            let entrada = subdir.unwrap().path();
            let experimental = entrada.as_path();

            if let Some(extensao) = experimental.extension()
            {
               if extensao == OsStr::new("xml") {
                  arquivos_xml.push(entrada.to_path_buf());
                  break;
               }
            }
         }
      }
   }

   // Apenas o diretório é relevante.
   for item in arquivos_xml.iter_mut()
      { item.pop(); }

   // retorna todos XML's encontrados.
   return arquivos_xml;
}

pub struct SeletorDeTransicoes {
   // Aqueles que já foram selecionados.
   escolhidos: HashSet<TransicaoWallpaper>,

   // Conjuntos com todos as transições disponíveis.
   todos: HashSet<TransicaoWallpaper>
}

impl SeletorDeTransicoes 
{
   pub fn cria() -> Self {
      let mut lista = todos_diretorios_de_wallpapers();
      let capacidade = lista.len();
      let construtor = TransicaoWallpaper::cria;
      let iterador = lista.drain(..).map(|x| construtor(&x));
      let escolhidos = HashSet::from_iter(iterador);
      let todos = HashSet::with_capacity(capacidade);

      Self { escolhidos, todos }
   }
}

#[derive(Hash, Eq)]
pub struct TransicaoWallpaper {
   // Urna com todos wallpapers.
   diretorio: PathBuf,

   // Todos wallpapers carregados na instanciação:
   papeis: FilaImagens,

   // Tempo de apresentação.
   duracao: Duration,
   cronometro: Instant,

   // Atual papel de parede.
   atual: Option<PathBuf>
}

impl TransicaoWallpaper {
   pub fn cria(diretorio: &Path) -> Self 
   {
      let dir = diretorio.to_path_buf();
      let r = dir.as_path();
      let papeis = Self::captura_imagens_do_diretorio_e_embaralha(r);
      let duracao = Self::calculo_de_duracao(&papeis);


      Self { 
         diretorio:dir, papeis, duracao, 
         cronometro: Instant::now(), atual: None 
      }
   }

   pub fn total(&self) -> usize
      { self.papeis.len() }

   pub fn troca(&mut self) -> bool
   {
      let ha_papeis_de_paredes = self.papeis.len() > 0;
      let passou_o_tempo = self.cronometro.elapsed() > self.duracao;
      let acabou_de_ser_iniciado = {
         self.atual.is_none() && 
         ha_papeis_de_paredes
      };

      
      if passou_o_tempo && ha_papeis_de_paredes || acabou_de_ser_iniciado
      {
         self.atual = self.papeis.pop_front();
         self.cronometro = Instant::now();
         self.alterna_wallpaper_para_o_atual();
         return true;
      }
      false
   }

   pub fn atual<'a>(&'a self) -> Option<&'a PathBuf>
      { self.atual.as_ref() }

   pub fn finalizada(&self) -> bool
      { self.papeis.is_empty() }

   /// Tempo estimado para termino.
   pub fn termino_estimado(&self) -> Duration
      { self.duracao * (self.papeis.len() as u32) }
}

impl TransicaoWallpaper {
   fn embaralha_uma_deque(input: FilaImagens) -> FilaImagens
   {
      let ultimo = input.len() - 1;
      let mut output = input;

      for p in 1..=((ultimo + 1) / 2)
      {
         let x = sortear::usize(0..=ultimo);

         if (p - 1) == x { continue; }
            
         output.swap(p - 1, x);
      }
      output
   }

   fn captura_imagens_do_diretorio_e_embaralha(dir: &Path) -> FilaImagens
   {
      let mut output = FilaImagens::with_capacity(50);
      let mut caminho: PathBuf = dir.to_path_buf();
      let ultimo: usize;

      if let Ok(mut entradas) = dir.read_dir()
      {
         while let Some(Ok(entry)) = entradas.next() { 
            caminho = entry.path();

            if nao_e_um_arquivo_xml(&caminho)
               { output.push_back(caminho); }
         }
      }

      Self::embaralha_uma_deque(output)
   }

   ///  Computa a duração total da transição no diretório selecionado.
   fn calculo_de_duracao(lista: &FilaImagens) -> Duration
   {
      let sorteado = sortear::u64(15*60..=43*60);
      let total = lista.len();
      let duracao = Duration::from_secs(sorteado);

      duracao / total as u32
   }

   /** Seleciona os argumentos para o comando da função abaixo 
    * 'alterna_wallpaper_para_o_atual' baseado. */
   fn argumentos_pra_determinado_ambiente_grafico() 
     -> Option<(&'static str, &'static str)>
   {
      match env::var("XDG_CURRENT_DESKTOP")
      {
         Ok(ambiente_grafico) => {
            if ambiente_grafico == "ubuntu:GNOME"
               { Some(("org.gnome.desktop.background", "picture-uri")) }
            else if ambiente_grafico == "MATE"
               { Some(("org.mate.background", "picture-filename")) }
            else { None }
         } 
         Err(erro_msg) =>  None 
      }
   }

   /** Realiza transição para o atual walpaper selecionado. Ele retorna 
     * um valor lógico 'verdadeiro' caso a transição tenha sido realizada
     * com sucesso, e 'falso' caso contrário.
     */
   fn alterna_wallpaper_para_o_atual(&self) -> bool
   {
      let argumentos: [&str; 4];
      let caminho = match self.atual.as_ref() {
         Some(objetoref) => objetoref,
         None => { return false; }
      };
      // Tupla de argumentos levando em conta o ambiente gráficos.
      let tupla = Self::argumentos_pra_determinado_ambiente_grafico();

      if let Some((chave, atributo)) = tupla {
         argumentos = [
            "set", chave, atributo, 
            caminho.to_str().unwrap(),
         ];
      } else
         { return false; }

      /* Constituindo o comando que roda, então executando ele ...*/
      let resultado_da_execucao = Command::new("gsettings")
         .args(argumentos.into_iter())
         .spawn().unwrap().wait();

      match resultado_da_execucao {
         Ok(exitcode) =>
            { exitcode.success() }
         Err(_) => false
      }
   }
}

impl Debug for TransicaoWallpaper {
   /// Mostra na sequência os seguintes itens: nome, total de wallpapers
   /// restantes, percentual da exibição da atual imagem, o tempo total
   /// da transição deste tipo de imagem.
   fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      let quantidade_de_wallpapers = self.papeis.len();
      let percentual_da_imagem = {
         self.cronometro.elapsed().as_secs_f32() /
         self.duracao.as_secs_f32() * 100.0
      };
      let nome_do_wallpaper = {
         #[allow(non_snake_case)]
         let DEFAULT: &OsStr = OsStr::new("Nenhum");

         match self.atual.as_ref() {
            Some(path) => path.file_name().unwrap_or(DEFAULT),
            None => DEFAULT
         }
      };

      fmt.write_fmt(format_args!(
         "{{{:?}, {}, {:0.0?} | {:3.0}% | {}}}",
         &nome_do_wallpaper, &quantidade_de_wallpapers,
         self.duracao, percentual_da_imagem, 
         tempo_legivel_duration(self.termino_estimado(), true)
      ))
   }
}

impl PartialEq for TransicaoWallpaper {
   fn eq(&self, outro: &Self) -> bool
      { self.diretorio == outro.diretorio }
}



/** Verifica se tal arquivo não é um XML. Se assim for, então dado o 
 * contexto de uso desta função, provavelmente será uma imagem. */
fn nao_e_um_arquivo_xml(caminho: &Path) -> bool
{
   if let Some(retorno) = caminho.file_name()
   {
      if let Some(base) = retorno.to_str() 
         { !base.ends_with(".xml") }
      else 
         { false }
   } else 
      { false }
}

#[allow(non_snake_case)]
#[cfg(test)]
mod tests {
   use super::{TransicaoWallpaper, todos_diretorios_de_wallpapers, Duration};

   #[test]
   fn prototipo_de_transicao_wallpaper() {
      let mut inputs_a = todos_diretorios_de_wallpapers();
      let input = inputs_a.pop().unwrap();
      let mut output = TransicaoWallpaper::cria(&input);

      println!("{:?}", output);

      while !output.finalizada()
      {
         if output.troca() 
            { println!("Alternar wallpaper"); }

         println!("{:?}", output);
         std::thread::sleep(Duration::from_secs_f32(1.1));
      }
   } 

   #[test]
   fn varredura_por_diretorios_com_wallpapers() {
      let funcao = todos_diretorios_de_wallpapers;

      println!("Tudo que foi encontrado:");
      for X in funcao().drain(..) 
         { println!("\t- {}", X.display()); }
   }
}
