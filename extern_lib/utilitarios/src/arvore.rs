/*
 * fazer uma árvore para visualização
 * de forma organizada, ramificando seus 
 * arquivos e sub-diretórios.
 */

// biblioteca externas:
extern crate termion;
use termion::terminal_size;
//use termion::color;

// biblioteca padrão do Rust.
use std::fs::read_dir;
use std::path::Path;

// meus módulos:
mod constroi_simbolos;
use constroi_simbolos::{matriciar_string,
                        matriz_para_string};


// tipos de galhos:
/// galho do tipo horizontal.
const GALHO_H:char = '\u{2500}';
/// galho vertical.
const GALHO_V:char = '\u{2502}';
/// galho conector vertical e horizontal.
const GALHO_VH:char = '\u{2570}';
/// conector entre dois verticais e um horizontal.
const GALHO_VHV:char = '\u{251c}';


fn desenha_trilha(esboco:&mut String, caminho:&Path , pfd:&mut u8) {
   /* escreva toda uma trilha, com ramificações
    * de sub-diretórios e arquivos, dado uma raiz
    * principal. */
   // navegando arquivos e diretórios.
   let lista_no_dir = match read_dir(caminho) {
       Ok(iterador) => iterador,
       Err(_) => panic!("diretório erro:\"{:#?}\"", caminho),
    };
   
   // navegando em seu conteúdo...
   for item in lista_no_dir {
      let item = item.unwrap();

      // possível link-símbolico.
      match item.path().as_path().read_link() {
          //se for link-simbólico, passar ele...
          Ok(sl) => {println!("é link-simbólico, burlando...{:#?}",sl); 
                    continue;},
          Err(_) => {} //apenas ignorando...
      };

      // string do caminho.
      let pth_str = item.path().into_os_string().into_string()
                    .expect("falha ao obter caminho no formato de string!");
      // nome do arquivo/diretório do caminho.
      let nome_pth = item.file_name().into_string().unwrap();

      // se for um diretório usar de recursividade.
      let espacamento = " ".repeat((*pfd) as usize);
      let pth = Path::new(pth_str.as_str());
      if pth.is_dir() {
          // molde de diretório(dois pontos).
          let mut str_aux = format!("{1}{2}{3}{4} {0}:\n",
                                    nome_pth,espacamento,
                                    GALHO_VH,GALHO_H,GALHO_H);
          // ajusta a string na tela.
          ajusta_string(&mut str_aux, true);
          esboco.push_str(str_aux.as_str());
          let novo_path = Path::new(pth_str.as_str());
          (*pfd) += 3; // cada chamada recursiva, aumenta a profundidade.
          desenha_trilha(esboco, novo_path, pfd);
          (*pfd) -= 3; // "volta" um diretório...
      }
      // se for apenas um arquivo, só registra.
      else {
         // molde diferente para arquivos:
         let mut straux = format!("{1}{2}{3}{4} \"{0}\"\n",
                                 nome_pth, espacamento,
                                 GALHO_VH,GALHO_H,GALHO_H);
         // ajusta a string na tela.
         ajusta_string(&mut straux, false);     
         esboco.push_str(straux.as_str());
      }
   }
}


/** retorna string representado tudo dentro de 
  um dado diretório, ramificando-os em arquivos
  e sub-diretórios. */
pub fn arvore(caminho:&str, mostra_arquivos:bool) -> String {
   // string para concatenar strings representado trilha.
   let mut trilha = String::new();
   // obtendo o nome do diretório raíz.
   let raiz_nome = match (*Path::new(caminho)).file_name() {
        Some(os_str) => match (*os_str).to_str() {
            Some(s) => s,
            _ => panic!("erroII"),
        },
        _ => panic!("erroI"),
    };

   // colocando raíz no começo...
   trilha.push_str(raiz_nome);
   trilha.push_str(":\n");

   // espaçar cada vez mais, em cada novo sub-diretório.
   let mut profundidade:u8 = 0;
   // raiz, de onde parte a trilhagem...
   let raiz = Path::new(caminho);
   // se estiver configurado para mostrar arquivos...
   if mostra_arquivos {
      desenha_trilha(&mut trilha, raiz, &mut profundidade);
   }
   else {
      desenha_trilha_dirs(&mut trilha, raiz, &mut profundidade);
   }

   // fazendo ajustes...
   let mut matriz_arv = matriciar_string(trilha.clone());
   preenchendo_galhos(&mut matriz_arv); 
   troca_galhos_adequadamente(&mut matriz_arv);
   preenche_primeira_coluna(&mut matriz_arv);
   //imprime(matriz_arv);

   // retorna string representando trilha.
   return matriz_para_string(&matriz_arv);
}


fn ajusta_string(s:&mut String, e_diretorio:bool) {
   /* caso uma string exceda a tela do terminal
    * a função vai reduzi-lá e implicitar que
    * tal string é mais extensa, continua... */
   let largura = match terminal_size() {
                 Ok(pair) => pair.0 as usize,
                 Err(_) => panic!("erro ao obter LARGURA do terminal"),
                 };

   // comprimento da string.
   let str_largura = s.len();
   if str_largura > largura {
      //let intervalo = (largura-4)..;
      if e_diretorio {
         s.replace_range((largura-8)..,"(...):\n");
      }
      else {s.replace_range((largura-2)..,"...\n");}
   }
}
   

fn desenha_trilha_dirs(esboco:&mut String, caminho:&Path , pfd:&mut u8) {
   /* escreva trilha, porém só registros diretórios...
    * arquivos não será ramificados na árvore. */
   // navegando arquivos e diretórios.
   let lista_no_dir = match read_dir(caminho) {
       Ok(iterador) => iterador,
       Err(_) => panic!("diretório erro:\"{:#?}\"", caminho),
    };
   
   // navegando em seu conteúdo...
   for item in lista_no_dir {
      let item = item.unwrap();

      // possível link-símbolico.
      match item.path().as_path().read_link() {
          //se for link-simbólico, passar ele...
          Ok(sl) => {println!("é link-simbólico, burlando...{:#?}",sl); 
                    continue;},
          Err(_) => {} //apenas ignorando...
      };

      // string do caminho.
      let pth_str = item
                    .path()
                    .into_os_string()
                    .into_string()
                    .expect("falha ao obter caminho no formato de string!");
      // nome do diretório do caminho.
      let nome_pth = item.file_name().into_string().unwrap();

      // se for um diretório usar de recursividade.
      let espacamento = " ".repeat((*pfd) as usize);
      let pth = Path::new(pth_str.as_str());
      if pth.is_dir() {
          // molde de diretório(dois pontos).
          let mut str_aux = format!("{1}{2}{3}{4} {0}:\n",
                                    nome_pth,espacamento,
                                    GALHO_VH,GALHO_H,GALHO_H);
          // ajusta a string na tela.
          ajusta_string(&mut str_aux, true);
          esboco.push_str(str_aux.as_str());
          let novo_path = Path::new(pth_str.as_str());
          (*pfd) += 3; // cada chamada recursiva, aumenta a profundidade.
          desenha_trilha_dirs(esboco, novo_path, pfd);
          (*pfd) -= 3; // "volta" um diretório...
      }
   }
}



/* patch de conserto da árvore */
fn _anterior_a_legenda(linha:&Vec<char>, coluna:u8) -> bool {
   for (p,ch) in linha.into_iter().enumerate() {
      if ch.is_ascii_alphanumeric() {
         return coluna < (p as u8);
      }
   }
   return false;
}

fn _vacuos_de_galho(matriz:&Vec<Vec<char>>) -> Vec<(u8,u8)> {
   // dimensão da matriz.
   let limite_linhas = matriz.len();
   let limite_colulas = matriz[0].len();
   // coordenadas dos lugares válido a sobreescrever.
   let mut vacuos:Vec<(u8,u8)> = Vec::new();

   for c in 0..limite_colulas {
      for l in 0..limite_linhas {
         if (l >= 1 && l <= limite_linhas-2) &&
            (c >= 1 && c <= limite_colulas-2) {
            /* cédulas a preencher com galhos sendo
             * selecionadas, levando em conta o seu
             * redor. */
            let prop_global = matriz[l][c].is_whitespace();
            let lugar_certo = _anterior_a_legenda(&matriz[l], c as u8);
            let caso1 = matriz[l-1][c] == GALHO_VH &&
                        matriz[l][c-1].is_whitespace() &&
                        matriz[l][c+1].is_whitespace() &&
                        matriz[l+1][c].is_whitespace() &&
                        prop_global && lugar_certo;

            let caso2 = matriz[l-1][c] == GALHO_VH &&
                        matriz[l][c+1].is_whitespace() &&
                        matriz[l][c-1].is_whitespace() &&
                        matriz[l+1][c] == GALHO_VH && 
                        prop_global && lugar_certo;

            let caso3 = matriz[l-1][c].is_whitespace() &&
                        matriz[l][c+1].is_whitespace() &&
                        matriz[l][c-1].is_whitespace() &&
                        matriz[l+1][c] == GALHO_VH && 
                        prop_global && lugar_certo;
            // cruz em branco.
            let caso4 = matriz[l-1][c].is_whitespace() &&
                        matriz[l][c+1].is_whitespace() &&
                        matriz[l][c-1].is_whitespace() &&
                        matriz[l+1][c].is_whitespace() &&
                        prop_global && lugar_certo;
            // adiciona coordenada.
            if caso1 || caso2 || caso3 || caso4 {
               vacuos.push((l as u8, c as u8));
            }
         }
      }
   }
   return vacuos;
}


fn acha_galho_dobrado(linha:&Vec<char>) -> usize {
   for indice in 0..linha.len() {
      if linha[indice] == GALHO_VH {
         //println!("\n\ncoluna={}\n{:#?}", indice, linha);
         return indice;
      }
   }
   panic!("não achou tal COLUNA!");
}


fn preenchendo_galhos(arvore:&mut Vec<Vec<char>>) {
   // dimensão da matriz:
   let max_y = arvore.len();
   //let max_x = arvore[0].len();
   // variável mutável para posição móvel da última linha.
   let mut l1;
   
   for l in 0..(max_y-1) {
      // última linha da matriz:
      l1 = (max_y-1)-l;
      // coluna com galho de dobra:
      let c = acha_galho_dobrado(&arvore[l1]);
      if c == 0 { continue; }

      /* subindo e trocando espaços até encontrar
       * outro galho de dobra.
       * proposições:   */
      let mut p1 = arvore[l1-1][c] != GALHO_VH; 
      let mut p2 = arvore[l1-1][c].is_whitespace();
      let mut p3 = !(arvore[l1-1][c+1].is_ascii_alphanumeric()
                     || arvore[l1-1][c+1] == '.' 
                     || arvore[l1-1][c+1] == '_');

      while p1 && p2 && p3 {
         // troca vácuo por galho vertical.
         arvore[l1-1][c] = GALHO_V;

         l1 -= 1; 
         // atualizando premissas
         p1 = arvore[l1-1][c] != GALHO_VH; 
         p2 = arvore[l1-1][c].is_whitespace();
         p3 = !(arvore[l1-1][c+1].is_ascii_alphanumeric()
               || arvore[l1-1][c+1] == '.');
      }
   }
}


fn troca_galhos_adequadamente(arvore:&mut Vec<Vec<char>>) {
   // dimensão da matriz:
   let max_y = arvore.len();
   let max_x = arvore[0].len();
   
   for l in 0..max_y-1 {
      for c in 0..max_x-1 {
         /* se o galho for dobrado(formato de L) 
          * e, o caractére abaixo for não branco
          * ou alfanumérico, ou seja, não altera
          * os galhos dobrados de ponta. */
         let condicao = {
            arvore[l][c] == GALHO_VH &&
            !(arvore[l+1][c].is_whitespace() ||
            arvore[l+1][c].is_ascii_alphanumeric())
         };
         // seguida a condição... faz troca.
         if condicao { arvore[l][c] = GALHO_VHV; }
      }
   }
}


fn preenche_primeira_coluna(arvore:&mut Vec<Vec<char>>) {
   let inicio = 1;  
   let mut fim = 1;

   // achando última dobradiça.
   for l in inicio..arvore.len() {
      if arvore[l][0] == GALHO_VH {
         if l > fim { fim = l; }
      }
   }

   // mudando... baeado no intervalo confiável filtrado.
   for l in inicio..fim {
      if arvore[l][0] == GALHO_VH {
         arvore[l][0] = GALHO_VHV;
      }
      else if arvore[l][0].is_whitespace() {
         arvore[l][0] = GALHO_V;
      }
   }
}



// ----------- testando funções --------

#[cfg(test)]
mod tests {
   // biblioteca padrão do Rust.
   use std::path::Path;
   use std::env::var;
   //use constroi_simbolos::imprime;
   
   // para testes.
   fn imprime(matriz:Vec<Vec<char>>) {
       // pega a linha da matriz.
       for row in matriz {
           // coluna na linha.
           for cell in row { print!("{}", cell); }
           print!("\n");
       }
   }
   
   #[test]
   fn lista_subdirs_de_foto() {
      let cmnh = match var("HOME") {
         Ok(st) => st + "/Documents",
         Err(_) => { panic!("variável de ambiente não existe!"); }
      };
      let pth = Path::new(cmnh.as_str());
      let mut trilha = String::from("");
      let mut p = 0;
      super::desenha_trilha_dirs(&mut trilha,pth, &mut p);
      println!("caminho:\n{}",trilha);
      assert!(true);
   }


   #[test]
   fn ramifica_ambos_modos() {
      let pth = var("HOME").unwrap() + "/Documents/códigos_rust";
      let primeiro = super::arvore(pth.as_str(),true);
      let segundo = super::arvore(pth.as_str(), false);

      println!("resultado(1):\n{}\n\nresultado(2):\n{}",primeiro,segundo);
      assert!(true);
   }


   #[test]
   #[ignore]
   fn coloca_galhos() {
      let caminho = var("HOME").unwrap() + "/Documents/códigos_rust";
      //let caminho = "/home/savio/Documents/códigos_rust";
      let arv = super::arvore(caminho.as_str(),false);
      let mut matriz_arv = super::matriciar_string(arv);
      super::preenchendo_galhos(&mut matriz_arv);
      imprime(matriz_arv);
      assert!(true);
   }

   #[test]
   #[ignore]
   fn conserta_galhos_desajustados() {
      let caminho = var("HOME").unwrap() + "/Documents/códigos_rust";
      let arv = super::arvore(caminho.as_str(),true);
      let mut matriz_arv = super::matriciar_string(arv);
      super::preenchendo_galhos(&mut matriz_arv);
      super::troca_galhos_adequadamente(&mut matriz_arv);
      imprime(matriz_arv);
      assert!(true);
   }

   #[test]
   fn testando_arvore_implementacao_terminada() {
      let nucleo = match var("HOME") {
         Ok(s) => s,
         Err(_) => { panic!("não existe tal variável!"); },
      };
      let caminho = nucleo.clone() + "/Videos";
      let arv1 = super::arvore(caminho.as_str(), true);
      let caminho = nucleo.clone() + "/Documents/códigos";
      let arv2 = super::arvore(caminho.as_str(),true);

      println!("{}\n\n{}\n",arv1, arv2);

      assert!(true);
   }
}
