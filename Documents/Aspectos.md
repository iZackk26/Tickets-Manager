# Aspectos del Proyecto

## Aspectos Generales
	* Paralelismo, las consultas al servidor deben ser concurrentes y el servidor debe poder procesar distintas consultas al mismo tiempo. Además, el estado de los asientos se debe reflejar en consultas paralelas.
	* Uso de sockets para enviar información al cliente.

## Estructura de Datos para el Estadio
	* Estadio: es una lista de zonas.
		* Zonas: nombre (Norte, Sur, Este, Oeste) y va a tener una lista de categorías.
			* Categoría: nombre (A, B, C, D) y va a tener una lista de filas.
				* Filas: nombre (V, W, X, Y, Z) y va a tener una lista de asientos.
					* Asientos: enumerados del 1 al 10. Tiene porcentaje de visibilidad, además un ENUM dedicado al estado de los asientos de la siguiente forma: ENUM { reservado, comprado, disponible }


	En resumen, el estadio está compuesto por 4 zonas, las cuales tienen 4 zonas cada una, cada zona está compuesta por 5 filas y cada fila por 10 asientos los cuales tienen las propiedades de visiblidad y su estado correspondiente.

## Ideas para el algoritmo

### Caso en el que se solicita más de un asiento
Primero hay que saber si se va a buscar en graderias con sombra (Norte y Sur) o con sol (Este y Oeste).
Se empieza buscando en la primera zona, se mete a la primera categoria y crea una lista con todos los asientos disponibles (separados por fila), la lista se veria algo así asientos disponibles de la categoria actual = [ [asientos disponibles de la fila W], [X], [Y], [Z]]. Si la cantidad de asientos solicitados es menor a la longitud total de esa lista, se tiene que pasar de categoria y así sucesivamente hasta encontrar una categoria que lo cumpla. Aquí surgen dos casos

* Nunca se encontró una categoria que cumpliese la condición:
	Se tienen que elegir los mejores asientos disponibles en cuanto a visibilidad, a pesar de que estén en categorias distintas
* Se encuentra alguna categoria que lo cumpla (caso más comun)
	Se crea una lista de candidatos para la zona, va a tener al mejor candidato de cada categoria
	Se crea una lista de candidatos para la categoria, de los cuales se va a escoger el mejor al final de analizar toda la categoria
	Se ingresa a la categoria y se tiene que preguntar si hay alguna fila con la cantidad de asientos disponibles que se están solicitando (esto se puede saber gracias a la lista que mencioné antes). De aquí surgen dos casos:
	* Ninguna fila satisface la cantidad de asientos que se están pidiendo
		Se deben elegir los espacios que menor distancia entre filas tengan, es decir, los más juntos posibles en cuanto a filas y se agrega a la lista de candidatos de zona directamente, sin hacer el analisis de candidatos por filas (ya que se va a escoger una combinacion de filas)
	* Existen filas que tienen la cantidad de asientos que se están solicitando
		Se deben elegir los asientos con menor distancia en la misma fila y ese conjunto de asientos se agrega a la lista de candidatos de la categoria
	Al final, si se cumplió la segunda condición, se escoge la mejor opción de los candidatos de la categoria y este se pasa a la lista de candidatos, esto se debe hacer por cada categoria. Finalmente, de la lista de candidatos para la zona, se debe escoger al mejor candidato, que serían los mejores asientos de toda la zona (Norte por ejemplo), luego se tiene que repetir este proceso para la otra zona (Sur por ejemplo) y finalmente se elige al mejor candidato entre el mejor de la Norte y el mejor de la Sur.

### Caso en el que se solicita un solo asiento
Si solo se solicita uno, se debe recorrer asiento por asiento, de cada fila y cada categoria y guardar al mejor del momento en una variable de "mejor temporal" (basada meramente en visibilidad), la cual si se encuentra un mejor candidato se remplaza por ese y al final de haber recorrido todo el estadio, el actual "mejor temporal" queda como el mejor asiento disponible.

### Aspectos a considerar
Para saber la cantidad de asientos de diferencia en la misma fila se puede tener un contador de diferencia que sea la cantidad de espacios entre cada asiento.
En el caso en el que se deba comparar por fila, se puede aplicar lo anterior sumando un contador de diferencia por cada fila, ya que al final una fila es solo un asiento más de diferencia.