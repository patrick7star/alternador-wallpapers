#!/usr/bin/python3 -O
"""
   Ao invés de copiar e renomear toda vez as bibliotecas rust compiladas, de
 forma puramente manual, em uma dada arquitetura, para algum subdiretório da 
 pasta 'lib' do Rust referente a tal arquitetura é claro. Iremos automatizar
 tal processo com um script. 
"""

import platform, shutil, os, glob, unittest
from pathlib import (PosixPath as Path)


sistema = platform.system().lower()
maquina = platform.machine()
nome_do_diretorio = "{}_{}".format(sistema, maquina)
DESTINO = Path("lib", nome_do_diretorio, "binaries")
if __debug__:
	DIRETORIO_FONTE = Path("target/debug/deps")
else:
	DIRETORIO_FONTE = Path("target/release/deps")
PADRAO = DIRETORIO_FONTE.joinpath("lib*.rlib")

if __debug__:
   existencia = DIRETORIO_FONTE.exists()

   print("""
      \rQual é a atual máquina: {}
      \rEle existe '{}'? {}
      \rFonte de bibliotecas compiladas existe? {}
      """.format(maquina, DESTINO.parent, DESTINO.exists(), existencia
      )
   )

   print("\nTodos candidatos a copia:")
   for entry in glob.glob(str(PADRAO)):
      print("\t\b\b\b-", entry)
   print("")

def cria_diretorio_se_necessario() -> None:
   # Verifica se o diretório já é existente, se não, então o cria.
   try:
      os.makedirs(DESTINO)
   except FileExistsError:
      print("O diretório '{}' já existe.")
   finally:
      assert(DESTINO.exists())


def realiza_copia_de_todos_bibliotecas_disponiveis() -> None:
   contador = 0

   print("Realizando copia dos binários ...")

   for caminho in glob.glob(str(PADRAO)):
      caminho = Path(caminho)
      print('\t\b\b-', caminho.name) 
      shutil.copy2(caminho, DESTINO)
      contador += 1

   print("\n... para '%s'." % DESTINO)
   print("Todas %d libs foram copiadas com sucesso." % contador)

def apara_este_grande_id_do_nome(caminho: Path) -> Path:
   """
      Você pode observar, que há algo bem estranho concatenado ao seu nome. 
    Por isso, vamos arrancar isso, e deixar apenas o nome normal. Como está
    entre um ponto(.) e um traço(-), tal operação fica bastante razoável.
   """
   diretorio = caminho.parent
   atual_nome = str(caminho.name)
   # Vem pela esquerda para achar o traço, e pela direita pelo ponto.
   p = atual_nome.index('-')
   q = atual_nome.rindex('.')
   # Copia tudo, menos este trecho. Então, forma o novo caminho com tal.
   novo_nome = atual_nome[0:p] + atual_nome[q:]

   return diretorio.joinpath(novo_nome)

def renomea_todos_libs_copiadas() -> None:
   caminho = destino.joinpath("./*.rlib")
   lista_de_binarios = glob.glob(str(caminho))

   print("\nAplicando processo de renomeação ...")

   for entrada in lista_de_binarios:
      In = Path(entrada)
      Out = apara_este_grande_id_do_nome(In)

      # Pode pode também ser um caminho absoluto.
      In.rename(Out)
      print("\t\b\b{} ==> {}".format(In.name, Out.name))
      # Confirma renomeação.
      assert(not In.exists())
      assert(Out.exists())

   print("\n... processo terminado.\n")

def cria_hard_linques_sem_o_codigo() -> None:
   regex = DESTINO.joinpath("./*.rlib")
   lista_das_novas_fontes = glob.glob(str(regex))

   print(
      "\nLincado vários binários referente na base do diretório com um " + 
      "nome mais limpo."
   )

   for entrada in lista_das_novas_fontes:
      In = Path(entrada)
      Out = apara_este_grande_id_do_nome(In)
      # Reprocessa saída para ir um diretório acima. Faz um linque no
      # diretório 'pai' do subdiretório com binários.
      base = Out.parent.parent
      Out = base.joinpath(Out.name)

      # Pode pode também ser um caminho absoluto.
      Out.hardlink_to(In)
      print("\t\b\b'{}' \u27fe '{}'".format(In.name, Out.name))

      # Confirma renomeação.
      assert(Out.exists())

   print("\n... processo terminado.\n")


class Unitarios(unittest.TestCase):
   def setUp(self):
      if DESTINO.exists():
         shutil.rmtree(DESTINO)
         print("Restante da última execução removido.")

   def tearDown(self):
      existencia = DESTINO.exists()
      print("Caminho '{}' ainda existe? {}".format(DESTINO, existencia))
      self.assertTrue(DESTINO.exists())
      print("\t\b\bremovendo", end=" ... ")
      shutil.rmtree(DESTINO)
      self.assertFalse(DESTINO.exists())
      print("feito.")
      shutil.rmtree(DESTINO.parent)
      self.assertFalse(DESTINO.exists())
      
   def construcao_do_script(self):
      cria_diretorio_se_necessario()
      realiza_copia_de_todos_bibliotecas_disponiveis()
      #renomea_todos_libs_copiadas()
      cria_hard_linques_sem_o_codigo()

if __name__ == "__main__":
   print(
      """
      \rProcesso de cópia das 'libs' que foram geradas na compilação nesta 
      \rmáquina. Assim, não é preciso refazer isso sempre, nesta máquina, ou 
      \rem outras com configuração parecida."""
   )
   cria_diretorio_se_necessario()
   realiza_copia_de_todos_bibliotecas_disponiveis()
   cria_hard_linques_sem_o_codigo()


