use caja_supermercado_multiarchivos::models::modelo_compras;
use caja_supermercado_multiarchivos::models::modelo_pagos;
use caja_supermercado_multiarchivos::operaciones::compras;
use caja_supermercado_multiarchivos::operaciones::pagos;

// Es importante tomar en cuenta que
// como ahora estamos en otro folder que no es un modulo
// entonces no es necesario utilizar el mod tests o el
// #[cfg(test)]
#[test] // basta con agregar el macro tests
fn cliente_paga_con_tarjeta_de_credito() {
    // Primero vamos a crear nuestro array de items
    let mut items_compra: Vec<modelo_compras::Item> = Vec::new();

    // Ahora vamos a agregar los 2 items
    let item1: modelo_compras::Item = modelo_compras::Item {
        nombre: String::from("item 1"),
        precio_unitario: 50.00,
        cantidad: 1.00
    };

    let item2: modelo_compras::Item = modelo_compras::Item {
        nombre: String::from("item 2"),
        precio_unitario: 150.00,
        cantidad: 2.00
    };

    // Agregando los items
    compras::agregar_item(&mut items_compra, item1);
    compras::agregar_item(&mut items_compra, item2);

    // Aca hacemos las primeras verificacione por ejemplo
    // verificamos que en efecto haya unicamente dos items
    assert_eq!(items_compra.len(), 2);

    // si queremos podemos probar que el total es 350.00
    assert_eq!(compras::total_compra(&items_compra), 350.00);

    // Ahora probamos en pago como dice el caso de prueba

    let resultado_pago = pagos::pagar(
        modelo_pagos::MetodoDePago::Tarjeta, // metodo de pago
        compras::total_compra(&items_compra), // total a pagar
        350.00, // recibido del cliente
        "12345" // numero de tarjeta
    );

    let resultado_esperado = modelo_pagos::ResultadoPago {
        metodo_pago: String::from("Tarjeta"),
        fue_exitoso: true,
        cambio: 0.0
    };

    // Ahora verificamos que los resultados del pago sean los esperados
    assert_eq!(resultado_pago.metodo_pago, resultado_esperado.metodo_pago);
    assert_eq!(resultado_pago.fue_exitoso, resultado_esperado.fue_exitoso);
    assert_eq!(resultado_pago.cambio, resultado_esperado.cambio);
}

#[test]
fn cliente_paga_en_efectivo_y_recibira_cambio() {
 // Primero vamos a crear nuestro array de items
    let mut items_compra: Vec<modelo_compras::Item> = Vec::new();

    // Ahora vamos a agregar los 2 items
    let item1: modelo_compras::Item = modelo_compras::Item {
        nombre: String::from("item 1"),
        precio_unitario: 50.00,
        cantidad: 1.00
    };

    let item2: modelo_compras::Item = modelo_compras::Item {
        nombre: String::from("item 2"),
        precio_unitario: 150.00,
        cantidad: 1.00
    };

    let item3: modelo_compras::Item = modelo_compras::Item {
        nombre: String::from("item 3"),
        precio_unitario: 150.00,
        cantidad: 1.00
    };

    // Agregando los items
    compras::agregar_item(&mut items_compra, item1);
    compras::agregar_item(&mut items_compra, item2);
    compras::agregar_item(&mut items_compra, item3);

    // Aca hacemos las primeras verificacione por ejemplo
    // verificamos que en efecto haya unicamente tres items
    assert_eq!(items_compra.len(), 3);

    // Ahora vamos a quitar un item y vamos a verificar que el total de compra sean 300.00
    compras::quitar_item(&mut items_compra, 0); // quitando el primer item (item1)
    assert_eq!(items_compra.len(), 2); // solo quedan 2 items
    // probar que el total es 300.00
    assert_eq!(compras::total_compra(&items_compra), 300.00);

    // Ahora probamos en pago como dice el caso de prueba

    let resultado_pago = pagos::pagar(
        modelo_pagos::MetodoDePago::Efectivo, // metodo de pago
        compras::total_compra(&items_compra), // total a pagar
        350.00, // recibido del cliente
        "" // numero de tarjeta
    );

    let resultado_esperado = modelo_pagos::ResultadoPago {
        metodo_pago: String::from("En Efectivo"),
        fue_exitoso: true,
        cambio: 50.0
    };

    // Ahora verificamos que los resultados del pago sean los esperados
    assert_eq!(resultado_pago.metodo_pago, resultado_esperado.metodo_pago);
    assert_eq!(resultado_pago.fue_exitoso, resultado_esperado.fue_exitoso);
    assert_eq!(resultado_pago.cambio, resultado_esperado.cambio);

}
