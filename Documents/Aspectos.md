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