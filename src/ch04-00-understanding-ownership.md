<!-- ia-translate: true -->
# Entendendo Ownership {#understanding-ownership}

Ownership é a funcionalidade mais característica do Rust e tem implicações profundas no
restante da linguagem. Ela permite que o Rust ofereça garantias de segurança de
memória sem precisar de um coletor de lixo (garbage collector), por isso é
importante entender como o ownership funciona. Neste capítulo, vamos falar sobre
ownership e também sobre várias funcionalidades relacionadas: borrowing, slices
e como o Rust organiza os dados na memória.
