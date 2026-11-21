# Extensión Drizzle para VS Code

Drizzle resalta la sintaxis de los archivos `.zz`, `.drizzle` y `.rd` en VS Code.

![Drizzle](../../images/logo_drizzle_language.png)

## Demostración

![Demostración](../../images/drizzle_demo.png)

## Ejemplo de control de trenes

```drizzle
&& Train control requirements
Estimate @0,0 = /Accurate estimate\nof speed & position/
Tracking @1,0 = <Tracking system>
Train @2,0 = [Train]
Duty @0,1 = {Train controller responsibility\nCompute safe acceleration for all trains between this station and the next.}
Safe @0,2 = /Safe command\nmessage/
Controller @1,2 = <Train controller>
Timing @0,3 = /Command in time/
Send @1,3 = (Send\ncommand)
Command @2,3 = [Command]

Estimate __ Tracking [o]
Tracking __> Train : currentSpeed\ncurrentLocation
Tracking <__ Train : measuredSpeed\nmeasuredLocation
Train __> Controller : As monitor\nmeasuredSpeed\nmeasuredLocation
Controller __> Command : As control\ncommandedSpeed\ncommandedAccel
Safe __ Controller [o]
Timing __ Controller [o] : Responsibility
Controller __ Send [o] : Performance
Duty --> Controller
```

## Salida del compilador

![Salida del compilador](../../output.png)

Consulta la [guía de sintaxis de Drizzle](./README.md#drizzle-syntax) para ver figuras, marcas de conexión y opciones de disposición.

## Desarrollar y probar la extensión

Abre la carpeta `drizzle/` en VS Code. El manifiesto `package.json` indica la ubicación de la gramática. `syntaxes/drizzle.tmLanguage.json` contiene la gramática TextMate y `language-configuration.json` configura los comentarios y los pares de delimitadores.

Pulsa **F5** para abrir Extension Development Host. Abre un archivo de ejemplo y comprueba el resaltado y la configuración del lenguaje. Tras hacer cambios, vuelve a iniciar el anfitrión o recarga su ventana.

Instala la extensión desde Marketplace o usa **Extensions: Install from VSIX...** en VS Code para instalar un paquete VSIX. Consulta la [documentación de extensiones de VS Code](https://code.visualstudio.com/api) para desarrollar y publicar.

## Notas de publicación de la extensión

### 1.1.3

- Corregir las URL del logotipo, la demostración de sintaxis y la salida generada en Marketplace.
- Enlazar directamente la guía completa de Drizzle en el README del repositorio.

### 1.1.0 – 1.1.2

- Ampliar la gramática Pest con nodos nombrados, referencias adelantadas, etiquetas UTF-8, secuencias de escape, saltos de línea y diagnósticos con línea y columna. Conservar objetivos, riesgos y agentes anónimos y comentarios de línea completa.
- Añadir objetos, acciones, notas, anotaciones en nube, trapecios descriptivos y rombos. Admitir bordes gruesos en hojas como requisitos y supuestos.
- Añadir conexiones continuas y discontinuas, puntas de flecha opcionales, etiquetas, bucles y uniones compartidas. Dibujar círculos vacíos y rellenos, zigzags de conflicto y barras de obstáculo.
- Añadir cuatro direcciones de disposición, ubicación explícita en filas y columnas, rutas ortogonales y desvíos exteriores. Separar conexiones paralelas y unir flechas al contorno de las figuras.
- Actualizar la gramática de VS Code y el README de la extensión, sustituir la demostración GIF por PNG e incluir el VSIX de Drizzle 1.1.0.

### 1.0.0

Primera publicación de Drizzle.
