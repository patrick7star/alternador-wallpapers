"""
   O caso pensado aqui inicialmente, é compactar os binários da lib usadas 
 no processo de compilação manual. Ele separa tal por sistema, versão e
 máquina. Tal categorização, se necessário, pode mudar futuramente.
"""
import platform
from tarfile import (TarFile)
from glob import (glob as Glob)
from unittest import (TestCase)
from pathlib import (Path)


KERNEL_VERSAO = platform.uname().release
PROCESSADOR = platform.processor()
SISTEMA = platform.system().lower()


print(
   "Máquina: {}\nSistema: {}\nVersão: {}\n"
   .format(PROCESSADOR, SISTEMA, KERNEL_VERSAO)
)

class CompactacaoDosBinarios(TestCase):
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


if __name__ == "__main__":
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
   
