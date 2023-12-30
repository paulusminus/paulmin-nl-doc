+++
title = "In rust we trust"
+++

## In rust we trust

Linus Torvalds, de man die ooit begon met de ontwikkeling van de Linux kernel, gebruikt regelmatig het woord vertrouwen. Gezien de omvang
van het project is vertrouwen in de mensen waar hij mee werkt een voorwaarde. Dit wil absoluut niet zeggen dat hij iedereen vertrouwd.
Voor hij besloot om de broncode van zijn kernel openbaar te maken had hij zijn twijfels over zakenmensen die zouden kunnen profiteren van zijn werk.

De programmeertaal [rust] maakt ook gebruik van het opensource model. Er wordt door veel mensen aan gewerkt. Er zijn ook steeds meer bedrijven
die investeren in de ontwikkeling van de taal, de compiler, hulpmiddelen en bibliotheken. Wat begon als een sneeuwbal die gegooit is door een
programmeur die niet begreep waarom je C of C++ zou moeten gebruiken om een browser te kunnen schrijven, lijkt een lawine te worden. Het beleid
van de groep die de ontwikkeling van de taal aanstuurt is behoedzaam.

## Gestructureerd, object georiënteerd of functioneel

Implementatie van abstracte datatypen. Functies in een bibliotheek zijn alleen toegankelijk als ze publiek zijn gemaakt.
Polymorfisme wordt doorgaans geïmplementeerd door gebruik te maken van een enum. Rust kent geen overerving.

De meeste rust programmeurs vermijden loops. Hun voorkeur gaat uit naar werken met datastructuren die de [iterator] of [stream] trait implementeren.

Middels extensions methods en operator overloading is het werken met een api te versimpelen.


[rust]: https://www.rust-lang.org/
[iterator]: https://doc.rust-lang.org/std/iter/trait.Iterator.html
[stream]: https://docs.rs/futures/0.3.30/futures/prelude/trait.Stream.html