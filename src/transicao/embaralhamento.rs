
/* trazido para cá, por motivos de 
 * legibilidade refatoração.
 */
extern crate utilitarios;
pub use utilitarios::aleatorio::sortear;

fn swap<A>(lista: &mut Vec<A>, p1: usize, p2:usize) {
   let remocao = lista.remove(p1);
   lista.insert(p2, remocao);
}

// pega uma lista, e embalhara seus valores.
pub fn embaralha<X>(lista: &mut Vec<X>) {
   let mut tamanho = lista.len();
   let ultimo: u8 = (tamanho - 1) as u8;

   // se houver apenas dois elementos, pode trocar ou não.
   if tamanho == 2 {
      if sortear::bool() 
         { swap(lista, 0, 1); }
   } else if tamanho <= 1 {
      // apenas abandona o programa; nada a fazer.
      return ();
   } else {
      // faz o embaralho o "tamanho da lista" vezes.
      while tamanho > 0 {
         let i = sortear::u8(0..=ultimo);
         let j = sortear::u8(0..=ultimo);
         if j != i 
            { swap(lista, i as usize, j as usize); }
         tamanho -= 1;
      }
   }
}

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn testa_embaralha() {
      let mut array = vec![1,2,3,4,5];
      let copia = array.clone();
      embaralha(&mut array);
      assert!(dbg!(array) != copia)
   }
}
