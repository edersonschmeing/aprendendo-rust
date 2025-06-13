fn exemplo01() {
    let valor = Box::new(42);
    println!("Valor na heap: {}", valor);
    //42 é alocado na heap, e valor guarda o ponteiro para esse dado.
    //O Box<T> implementa Deref, então você pode usá-lo como se fosse a própria variável.
} 

fn dobra(valor: &mut Box<i32>) {
    **valor *= 2;
    //Primeiro * desreferencia o &mut Box<i32>,
    //Segundo * desreferencia o Box<i32> para acessar o i32.
}

fn exemplo02() {
    let mut num = Box::new(10);
    dobra(&mut num);
    println!("Valor dobrado: {}", num);
}

struct MinhaCaixa;

impl Drop for MinhaCaixa {
    fn drop(&mut self) {
        println!("MinhaCaixa foi destruída!");
    }
    //O Box chama drop automaticamente quando sai 
    //do escopo — sem free() manual!
}

fn exemplo03() {
    let _caixa = Box::new(MinhaCaixa);
    println!("Antes de sair do escopo.");
}

fn exemplo04() {
     //Criando e usando ponteiros manualmente
    let x = Box::new(5); // aloca no heap
    let raw_ptr: *const i32 = Box::into_raw(x); // Aqui usamos Box::into_raw para obter um ponteiro bruto

    unsafe {
        println!("Valor via ponteiro: {}", *raw_ptr);
        // recupere a propriedade do Box (evita vazamento de memória)
         let _x = Box::from_raw(raw_ptr as *mut i32); //depois Box::from_raw para retomar o controle do valor
    }
}

fn main() {
    exemplo01();
    exemplo02();
    exemplo03();
    exemplo04();
}