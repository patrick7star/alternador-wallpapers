
/* Muitos módulos importam constantes uns do outro, sejam eles nomes
 * padrões, células em branco, caminhos para algum lugar, sei lá! Para
 * facilitar achar todos, vamos centralizar todos aqui. Todas elas serão
 * públicas, as que não forem mais utilizadas ganharam o rótulo de 
 * "código morto" ao invés da total exclusão. */

/* o máximo e mínimo de tempo que deve ser selecionada uma nova
 * transição- de-imagens é entre 5h e 8h. */
pub const MINIMO:u16 = 1_600;
pub const MAXIMO:u16 = 3_600;

/* caminho do diretório que será trabalhado. diretório onde será varrido 
 * por slides-de-transição. */
pub const RAIZ: &str = concat!(env!("HOME"), "/Pictures");

// registros de mudanças feitas.
#[allow(dead_code)]
pub const BD1: &str = concat!(
   env!("RUST_CODES"),
   "/alternador-wallpapers/data",
   "/ultima_escolha.txt"
);

/* caminho para novo arquivo que armazenará tais registro de data. */
pub const CAMINHO_ARQUIVO: &str = concat!(
   env!("RUST_CODES"),
   "/alternador-wallpapers/data",
   "/data_de_registro.dat" 
);

// atalho para o binário do Python.
pub const PYTHON: &'static str = "/usr/bin/python3";
pub type Str = &'static str;

pub const ARQUIVO_CONF: &str = concat!(
   env!("RUST_CODES"),
   "/alternador-wallpapers/data/",
   "configuracao.json"
);

// arquivo onde serão gravados.
pub const SELECOES_FEITAS: &str = concat!(
   env!("RUST_CODES"),
   "/alternador-wallpapers/data",
   "/historico_de_escolhas_feitas.txt"
);

// arquivo de configuração das 'Datas Especiais'.
#[allow(dead_code)]
pub const ARQUIVO_DE: &str = concat!( 
   env!("RUST_CODES"),
   "/alternador-wallpapers/data",
   "/datas_especiais.conf"
);

pub const ULTIMA_NOTIFICACAO: &str = concat!(
   env!("RUST_CODES"), 
   "/alternador-wallpapers/data",
   "/registro_notificação.txt"
);
