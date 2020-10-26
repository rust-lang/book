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
da parte di sviluppatori esperti. In Rust, il compilatore svolge il ruolo di guardiano rifiutando la
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

Rust è anche per le persone che vogliono aiutarci a costruire il linguaggio di programmazione Rust,
community, strumenti di sviluppo, e librerie. Noi saremmo felicissimi se voi decideste di contribuire.

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
senza fare distinzione tra di loro. Abbiamo provato a creare del materiale che fosse
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
vorrai lavorare su un progetto applicando i concetti che hai imparato.

Il Capitolo 5 discute le strutture e i metodi, e il Capitolo 6 copre le enum, le espressioni
`match`, e il costrutto di controllo del flusso `if let`. Userai strutture e
enum per creare tipi personalizzati in Rust.

Nel Capitolo 7, imparerai circa il sistema di moduli di Rust e le regole di privacy
per organizzare il tuo codice e le sue Application Programming Interface
(API) pubbliche. Il Capitolo 8 parla di alcune tra le più comuni strutture dati
che la libreria standard fornisce, come i vettori, le stringhe, e le hash map. Il Capitolo 9
esplora la filosofia e le tecniche della gestione degli errori in Rust.

Il Capitolo 10 si fa strada attraverso: tipi generici, tratti e lifetimes, i quali rendono possibile
creare codice che funzioni per più tipi diversi. Il Capitolo 11 è concentrato sul testing,
che nonostante le garanzie di sicurezza di Rust è necessario per assicurasi che
la logica del codice sia corretta. Nel Capitolo 12, creeremo una nostra implementazione di un 
sottoinsieme di funzionalità del comando da terminale `grep` che cerca del testo
all'interno di files. Per fare questo, useremo gran parte degli argomenti di cui avremo parlato nei
capitoli precedenti.

Il Capitolo 13 esplora le chiusure e gli iteratori: funzionalità di Rust prese dai
linguaggi di programmazione funzionali. Nel Capitolo 14, esamineremo Cargo più nel dettaglio
e parleremo delle best practices per condividere le tue librerie con altri.
Il Capitolo 15 discute dei puntatori intelligenti che la libreria standard mette a disposizine
e i tratti che attivano la loro fiunzionalità.

Nel Capitolo 16, analizzeremo diversi modelli di programmazione concorrente
e parleremo di come Rust ti aiuti a programmare in multi-threading senza paura.
Il Capitolo 17 mostra come associare gli idiomi di Rust ai principi della
programmazione ad oggetti con la quale potresti sentirti più a tuo agio.

Il Capitolo 18 è una guida ai pattern e al pattern matching, che sono molto potenti
per esprimere idee attraverso i programmi in Rust. Il Capitolo 19 contiene 
un'ampia varietà di argomenti interessanti, compresi; Rust non sicuro, macro,
ulteriori informazioni sui lifetimes, tratti, tipi, funzioni, e chiusure.

Nel Capitolo 20, completeremo un prgetto in cui implementeremo un web server
multithread a basso-livello!

In ultimo, alcune appendici contententi informazioni utili sul linguaggio in
formato guida. L'Appendice A copre le keywords di Rust, la B copre gli operatori e i simboli, 
l'Appendice C copre i tratti derivabili forniti dalla libreria standard,
la D parla di alcuni utili strumenti di sviluppo e la E mostra le varie edizioni di Rust.

Non c'è un modo sbagliato di leggere questo libro. Se vuoi saltare avanti, fallo!
Potrai sempre tornare indietro se dovessi sentirti perso o confuso.
Fai tutto ciò che funziona per te.

<span id="ferris"></span>

Una parte importante nel processo di studio di Rust è imparare come leggere
i messaggi di errore forniti dal compilatore: questo ti guiderà verso il codice giusto.
Per tanto, forniremo tanti esempi che non compileranno con i relativi messaggi di errore
che il compilatore ti motrerà in ciascuna situazione. Sappi che se prendi 
ed esegui un esempio random, potrebbe non compilare! Assicurati di leggere
il testo circostante per capire quale errore vuole mostrare il codice che
stai provando a eseguire. Ferris ti aiuterà a distinguere il codice che è stato scritto per non funzionare:

| Ferris                                                                 | Significato                                      |
|------------------------------------------------------------------------|--------------------------------------------------|
| <img src="img/ferris/does_not_compile.svg" class="ferris-explain"/>    | Questo codice non compila!                       |
| <img src="img/ferris/panics.svg" class="ferris-explain"/>              | Questo codice fornisce errori di runtime!        |
| <img src="img/ferris/unsafe.svg" class="ferris-explain"/>              | Questo blocco contiene codice pericoloso.        |
| <img src="img/ferris/not_desired_behavior.svg" class="ferris-explain"/>| Questo codice non esegue il funzionamento voluto.|

Nella maggior parte delle situazioni ti condurremo alla versione corretta di qualsiasi codice
che non compila.

## Source Code

I file sorgenti dai quali questo libro viene generato possono essere trovati su
[GitHub][book].

[book]: https://github.com/Ciro-Fusco/book_it/tree/master/src
