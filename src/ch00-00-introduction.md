# Introduzione

> Nota: Questa edizione del libro è la stessa di [The Rust Programming
> Language][nsprust] disponibile in formato cartaceo o ebook su [No Starch
> Press][nsp] in lingua inglese.

[nsprust]: https://nostarch.com/rust
[nsp]: https://nostarch.com/

Benvenuti in *Programmare in Rust*, un libro introduttivo su Rust.
Il linguaggio di programmazione Rust ti permette di scrivere 
codice più veloce e software più affidabile.
La comodità dei linguaggi di alto-livello e il controllo a basso-livello sono spesso in contrasto
nel design di linguaggi di programmazione; Rust affronta questo problema. Attraverso il bilanciamento
di potenti capacità tecniche e l'esperienza di grandi sviluppatori, Rust ti dà l'opportunità
di controllare dettagli di basso-livello (come l'uso della memoria) senza tutti i fastidi
tradizionalmente associati a questo tipo di controllo.

## Per chi è Rust

Rust è l'ideale per molte persone per svariati motivi. Guardiamo alcuni
dei gruppi più importanti.

### Teams di Sviluppatori

Rust sta dimostrando di essere un utile strumento per collaborare tra grandi teams
di sviluppatori con diversi livelli di conoscenza di programmazione di sistemi. Il codice 
di basso-livello è incline ad avere diversi subdoli bug, che nella maggior parte degli
altri linguaggi possono essere trovati solo attraverso lunghi testing e attente revisioni del codice 
da parte di sviluppatori esperti. In Rust, il compilatore svolge il ruolo di guardianoby rifiutando la
compilazine di codice contenente questi bug sfuggenti, inclusi i bug di concorrenza. Lavorando
insieme al compilatore, il team può spendere il proprio tempo concentrandosi sulla logica di
programma piuttosto che rincorrere i bugs.

Rust inoltre usa strumenti di sviluppo contemporanei al mondo della programmazione di sistemi:

* Cargo, gestore delle dipendenze e strumento di build, in aggiunta,
  compila, e gestisce le dipendenze in maniera indolore e uniforme attraverso tutto
  l'ecosistema di Rust.
* Rustfmt assicura uno stile di scrittura uniforme tra gli sviluppatori.
* Il Rust Language Server supporta l'integrazione per il completamento del codice
  e la segnalazione di errori inline, negli Integrated Development Environment (IDE).

Usando questi strumenti ed altri nell'ecosistema di Rust, gli sviluppatori
possono essere produttivi mentre scrivono codice al livello di sistema.

### Studenti

Rust è per gli studenti e per coloro che sono interessati ad apprendere concetti 
sui sistemi. Usando Rust, diverse persone hanno appreso riguardo argomenti come lo
sviluppo di sistemi operativi. La community è ben accetta e felice di rispondere a domande
poste dagli studenti. Attraverso sforzi come questo libro, il team di Rust vuole
rendere i concetti sui sistemi più accessibili a tutti, soprattutto a coloro
che si avvicinano ora alla programmazione.

### Aziende

Centinaia di aziende, piccole e grandi, usano Rust nello sviluppo di svariati
tasks. Alcuni dei quali includono tools da linea di comando, servizi web, strumenti di DevOps,
embedded devices, audio and video analysis and transcoding, cryptocurrencies,
bioinformatica, motori di ricerca, applicazioni di Internet of Things, machine
learning, e anche una vasta porzione del web browser Firefox.

### Sviluppatori Open Source

Rust è anche per le persone che vogliono aiutarci a costruire il linguaggio di programmazione Rust, community,
strumenti di sviluppo, e librerie. Noi saremmo felicissimi se voi decideste di contribuire.

### Persone che Desiderano Velocità e Stabilità

Rust è per coloro che bramano velocità e stabilità da un linguaggio. Per velocità,
intendiamo la velocità dei programmi che tu puoi creare e la velocità con cui
Rust ti permette di scriverli. I controlli del compilatore di Rust assicurano la stabilità
attraverso l'aggiunta di features e il refactoring. Questo si oppone alla fragilità del
codice legacy nei linguaggi senza questi controlli, e che gli sviluppatori hanno spesso 
paura di modificare. Impegnandoci ad avere astrazioni a costo zero, funzionalità alto-livello
che vengano compilate in codice basso-livello veloce come il codice scritto manualmente, Rust
si sforza di creare del codice sicuro e che sia anche veloce.

Rust spera di supportare anche molti altri tipi di utenti; quelli menzionati
sono solo una parte dei tantissimi interessati. Soprattutto, la più grande 
ambizione di Rust è quella di eliminare tutti i compromessi che i programmatori hanno dovuto
accettare per decenni garantendo sicurezza *e* produttività, velocità *e* comodità. Dai
a Rust una possibilità e scopri se questa scelta è adatta a te.

## Per Chi è Questo Libro

Questo libro dà per scontato che tu abbia già programmato in altri linguaggi ma
senza fare distinzione tra quale. Abbiamo provato a creare del materiale che fosse
di facile comprensione per tutti indipendentemente dal linguaggio di progrmmazione di provenineza. 
Non spenderemo molto tempo parlando di cosa *è* la programmazione o su come affrontarla. 
Se sei completamente nuovo alla programmazione, faresti meglio a leggere
un libro che ti fronisca specificamente un'introduzione alla programmazione.

## Come Usare Questo Libro

Principalmente, questo libro presuppone che tu lo stia leggendo in sequenza dal fronte
al retro . I capitoli successivi si basano su concetti espressi nei capitoli precedenti,
e questi ultimi potrebbero non scendere nei dettagli di ogni argomento; tipiacmente riaffrontiamo
l'argomento nei capitoli successivi.

Troverai due tipi di capitoli in questo libro: capitoli concettuali e
capitoli progettuali. Nei capitoli concettuali, imparerai nozioni riguardo gli aspetti di Rust.
Nei capitoli progettuali, creeremo piccoli progetti insieme, applicando quello che hai imparato
fino a quel momento. I Capitoli 2,12 e 20 sono progettuali; i restanti sono concettuali.

Il Capitolo 1 spiega come installare Rust, come scrivere un programma "Hello, world!",
e come usare Cargo, il gestore di pacchetti e strumento di compilazione di Rust. Il Capitolo 2
è un'introduzione pratica al linguaggio Rust. Lì Parleremo di concetti ad alto-livello,
e i capitoli successivi forniranno maggiori dettagli. Se vuoi sporcarti subito
le mani, il Capitolo 2 è il posto adatto per te. Inizialmente, potresti
saltare il Capitolo 3, che copre delle funzionalità di Rust simili
a quelle di altri linguaggi di programmazine, e puntare direttamente al Capitolo 4
per imparare il sistema di "proprietà" di Rust (ownership). Comunque, se sei uno studente
meticoloso che preferisce imparare ogni singolo dettaglio prima di spostarsi al successivo,
potresti voler saltare il Capitolo 2 e andare direttamente al Capitolo 3, tornadno al 2 quando
vorrai lavorare su un progetto applicando i dettagli che hai imparato.

Il Capitolo 5 discute le strutture e i metodi, e il Capitolo 6 copre le enum, le espressioni
`match`, e il costrutto di controllo del flusso `if let`. Userai strutture e
enum per creare tipi personalizzati in Rust.

Nel Capitolo 7, imparerai circa il sistema di moduli di Rust e le regole di privacy
per organizzare il tuo codice e le sue Application Programming Interface
(API) pubbliche. Il Capitolo 8 parla di alcune tra le più comuni strutture dati
che la libreria standard fornisce, come i vettori, le stringhe, e le hash map. Il Capitolo 9
esplora la filosofia e le tecniche della gestione degli errori in Rust.

Chapter 10 digs into generics, traits, and lifetimes, which give you the power
to define code that applies to multiple types. Chapter 11 is all about testing,
which even with Rust’s safety guarantees is necessary to ensure your program’s
logic is correct. In Chapter 12, we’ll build our own implementation of a subset
of functionality from the `grep` command line tool that searches for text
within files. For this, we’ll use many of the concepts we discussed in the
previous chapters.

Chapter 13 explores closures and iterators: features of Rust that come from
functional programming languages. In Chapter 14, we’ll examine Cargo in more
depth and talk about best practices for sharing your libraries with others.
Chapter 15 discusses smart pointers that the standard library provides and the
traits that enable their functionality.

In Chapter 16, we’ll walk through different models of concurrent programming
and talk about how Rust helps you to program in multiple threads fearlessly.
Chapter 17 looks at how Rust idioms compare to object-oriented programming
principles you might be familiar with.

Chapter 18 is a reference on patterns and pattern matching, which are powerful
ways of expressing ideas throughout Rust programs. Chapter 19 contains a
smorgasbord of advanced topics of interest, including unsafe Rust, macros, and
more about lifetimes, traits, types, functions, and closures.

In Chapter 20, we’ll complete a project in which we’ll implement a low-level
multithreaded web server!

Finally, some appendixes contain useful information about the language in a
more reference-like format. Appendix A covers Rust’s keywords, Appendix B
covers Rust’s operators and symbols, Appendix C covers derivable traits
provided by the standard library, Appendix D covers some useful development
tools, and Appendix E explains Rust editions.

There is no wrong way to read this book: if you want to skip ahead, go for it!
You might have to jump back to earlier chapters if you experience any
confusion. But do whatever works for you.

<span id="ferris"></span>

An important part of the process of learning Rust is learning how to read the
error messages the compiler displays: these will guide you toward working code.
As such, we’ll provide many examples that don’t compile along with the error
message the compiler will show you in each situation. Know that if you enter
and run a random example, it may not compile! Make sure you read the
surrounding text to see whether the example you’re trying to run is meant to
error. Ferris will also help you distinguish code that isn’t meant to work:

| Ferris                                                                 | Meaning                                          |
|------------------------------------------------------------------------|--------------------------------------------------|
| <img src="img/ferris/does_not_compile.svg" class="ferris-explain"/>    | This code does not compile!                      |
| <img src="img/ferris/panics.svg" class="ferris-explain"/>              | This code panics!                                |
| <img src="img/ferris/unsafe.svg" class="ferris-explain"/>              | This code block contains unsafe code.            |
| <img src="img/ferris/not_desired_behavior.svg" class="ferris-explain"/>| This code does not produce the desired behavior. |

In most situations, we’ll lead you to the correct version of any code that
doesn’t compile.

## Source Code

The source files from which this book is generated can be found on
[GitHub][book].

[book]: https://github.com/rust-lang/book/tree/master/src
