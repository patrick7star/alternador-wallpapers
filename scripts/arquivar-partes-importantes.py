"""
   O caso pensado aqui inicialmente, é compactar os binários da lib usadas 
 no processo de compilação manual. Ele separa tal por sistema, versão e
 máquina. Tal categorização, se necessário, pode mudar futuramente.
"""
import platform, gzip
from tarfile import (TarFile)
from glob import (glob as Glob)
from unittest import (TestCase)
from pathlib import (Path)
from os import (getenv)

PROCESSADOR = platform.processor()
SISTEMA = platform.system().lower()


def identificacao_do_computador_usado() -> None:
   KERNEL_VERSAO = platform.uname().release

   print(
      "Máquina: {}\nSistema: {}\nVersão: {}\n"
      .format(PROCESSADOR, SISTEMA, KERNEL_VERSAO)
   )

def compacta_biblioteca_externa_utils() -> None:
   """
   Função apenas funciona na máquina do desenvolvedor. Este que tem a mais
   nova versão de tal biblioteca. O 'tarball' que vem no diretório 'lib', 
   não é garantia que é a mais atualizada, entretanto, geralmente é o 
   suficiente.
   """
   NOME         = "utils.tar"
   DESTINO      = Path("./lib").joinpath(NOME);
   BASE         = getenv("RUST_CODES")
   ALVO         = Path(BASE).joinpath("rust-utilitarios")
   PADRAO_REGEX = str(ALVO) + "/*"
   EXCLUSOES    = ["target", "Cargo.lock"]

   # Proposiçoes:
   nao_tem_variavel_de_ambiente = (BASE is None)
   diretorio_nao_existe = (not ALVO.exists())

   if (nao_tem_variavel_de_ambiente or diretorio_nao_existe):
      print("Algo não base, então não é possível continuar.", file=stderr)
      exit(1)

   print("Listagem do que será, ou não, arquivado:")

   with TarFile(DESTINO, 'w') as arquivado:
      for item in Glob(PADRAO_REGEX):
         caminho = Path(item)
         nome_do_item = caminho.name

         if nome_do_item in EXCLUSOES:
            print("\t\b\b\b- {}\t[negado]".format(nome_do_item))
         else:
            print("\t\b\b\b- {}".format(nome_do_item))
            arquivado.add(item)

      print("\nO que foi propriamente arquivado em %s:" %DESTINO)

      for membro in arquivado.getnames():
         print('\t\b\b\b-', membro)

def compactacao_dos_binarios_compilados() -> None:
   nome = "{}_{}.tar".format(SISTEMA, PROCESSADOR)
   caminho = Path("./lib").joinpath(nome);
   arquivado = TarFile(caminho, 'w')

   print("Listagem do que será arquivado:")

   for item in Glob("./lib/*/*.rlib"):
      arquivado.add(item)

   print("Visualizando conteúdo interno ...")
   for membro in arquivado.getnames():
      print('\t\b\b\b-', membro)

   arquivado.close()

def ja_foi_comprimido(tarball: Path, compressoes: set) -> bool:
   "Verifica se algum dos itens selecionados já tem sua compressão." 
   rotina = (lambda path: path.name)
   todos_itens = set(map(rotina, compressoes))
   nome = tarball.name + ".gz"

   if nome in todos_itens:
      return True
   else:
      return False


def filtragem_das_compressoes() -> set:
   """
   O algoritmo consiste em três 'pipelines': aquele que separa o que já 
   foi comprimido, e o que pode ser; a parte dos que podem ser e já foram; 
   e a última, a exclusão dos que cairam na segunda iteração.
   """
   SUFIXO = "tar.gz"
   ENTRADAS = Glob("./lib/*")
   selecoes = set([])
   comprimidos = set([])
   fila = []

   # Iteração pra captura de itens a serem comprimidos, e também os a 
   # serem evitados:
   for caminho in ENTRADAS: 
      caminho = Path(caminho)

      print("\t\b\b\b-", caminho.name, end="\t")

      if caminho.is_dir():
         print("[é diretório]")
      elif caminho.name.endswith(SUFIXO):
         print("[já comprimido]")
         comprimidos.add(caminho)
      elif caminho.name.endswith(".tar"):
         selecoes.add(caminho)
      else:
         print('')

   # Iteração para realização dos itens a serém comprimidos:
   for item in selecoes:
      if ja_foi_comprimido(item, comprimidos):
         fila.append(item)

   for item in fila:
      selecoes.remove(item)

   return selecoes

def realiza_compressao_dos_tarballs() -> None:
   for caminho in Glob("./lib/*"):
      print(caminho)

class CompactacaoDosBinariosEsboco(TestCase):
   def setUp(self): 
     self.rejeito = Path("./lib")

   def tearDown(self):
      print("Caminho à deletar:", self.rejeito)
      self.assertTrue(self.rejeito.exists())
      self.rejeito.unlink()
      self.assertFalse(self.rejeito.exists())

   def runTest(self):
      nome = "{}_{}.tar".format(SISTEMA, PROCESSADOR)
      self.rejeito = self.rejeito.joinpath(nome);
      self.assertFalse(self.rejeito.exists())
      compilados = TarFile(self.rejeito, 'w')

      print("Listagem do que será arquivado:")

      for item in Glob("./lib/*/*.rlib"):
         compilados.add(item)

      compilados.close()
      self.assertTrue(self.rejeito.exists())

class CompactacaoDosUtilitarios(TestCase):
   def runTest(self):
      compacta_biblioteca_externa_utils()

class FiltragemDasConversoesEsboco(TestCase):
   def runTest(self):
      SUFIXO = "tar.gz"
      ENTRADAS = Glob("./lib/*")
      selecoes = set([])
      comprimidos = set([])

      # Iteração pra captura de itens a serem comprimidos, e também os a 
      # serem evitados:
      for caminho in ENTRADAS: 
         caminho = Path(caminho)

         print("\t\b\b\b-", caminho.name, end="\t")

         if caminho.is_dir():
            print("[é diretório]")
         elif caminho.name.endswith(SUFIXO):
            print("[já comprimido]")
            comprimidos.add(caminho)
         elif caminho.name.endswith(".tar"):
            selecoes.add(caminho)
         else:
            print('')

      fila = []
      # Iteração para realização dos itens a serém comprimidos:
      print("\nItens já comprimidos:", end=" ")
      for item in selecoes:
         if ja_foi_comprimido(item, comprimidos):
            print(item, end=", ")
            fila.append(item)
      print("\b\b")

      print("Antes:", selecoes)
      for item in fila:
         selecoes.remove(item)
      print("Depois:", selecoes)

if __name__ == "__main__":
   identificacao_do_computador_usado()
   compacta_biblioteca_externa_utils()
   compactacao_dos_binarios_compilados()
   
