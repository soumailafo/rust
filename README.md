# rust
TP rust

FOMBA Soumaila

https://github.com/soumailafo/rust



# Reponses aux Questions : Rappels de Rust, généralités

1/ En Rust à quoi servent les références ?

Une référence représente un emprunt d'une certaine valeur détenue.
Elles vous permettent de vous référer à une certaine valeur sans en prendre possession.

2/ Citez en Rust les grandes façons de déclarer ses propres types.

Les déclarations se font avec le mot clef **let ou let mut**

3/ Rust est compilé nativement (assembleur sous forme de code machine) ou compte sur une machine virtuelle pour s’exécuter ?

Selon les développeurs du langage, cela représente un grand pas pour la prise en charge de Windows
et rend plus facile la liaison entre le code Rust et le code compilé en utilisant la chaine de compilation native.

Rust est compilée nativement.

4/ Imaginons qu’on a un système avec un processeur 8bits, quelle est la valeur maximale adressable ? Écrire la solution en notation hexadécimale et décimale.

En decimal:     2^8 - 1 = 255

En héxadécimal:     2^8 - 1 = FF

5/ Donnez votre définition d’un processus citez vos sources !

Un processus sont des séquences d’instructions binaires manuelles ou automatiques, logées dans la mémoire d’un ordinateur et qui s’exécutent par le processeur (CPU) de ce dernier.

source: https://www.journaldunet.fr/web-tech/dictionnaire-du-webmastering/1445274-process-informatique-definition-detaillee-et-concrete/


# 2.1/Pratique - Micro-shell

1. Créer un projet binaire avec cargo :
    cargo new micro-shell
        Created binary (application) `micro-shell` packageedinline{shell}{}.
Comment compiler puis executer son programme ?: 

To compile: **cargo build**

To run: **cargo run**

Execute les test ?:
To test: **cargo test**

Ou sont rangé les binaires (en mode debug) ?
L'exécutable sera généré dans le dossier target/debug/.


# 3.1 Questions : Executer une commande

3/ Réussir à exécuter une commande avec std::process::Command::status.
Voir le code

4/ Afficher le statut d’une commande, pourquoi Rust vous force à récupérer le statut ? 
Rust vous force à récupérer le status pour savoir si l'execution a été un succès

5/ Que fait votre programme pendant que son enfant s’exécute ?
Pendant que child process s'execute, le programme attend sa fin et collecte son état de sortie.

6/ Maintenant gérer mais pour une commande avec plusieurs arguments !
Voir le code.

# 4.1 Questions : Redirections

7/ Donnez avec vos mot une définition d’un tupe entre deux programmes citez vos sources.
C'est un mécanisme qui permet de chaîner des processus de sorte que la sortie d'un processus (stdout) alimente directement l'entrée (stdin) du suivant.

Le programme programme1 est exécuté par le système qui envoie les résultats au programme2 qui à son tour renvoie les résultats sur la sortie standard du système.

Source: https://fr.wikipedia.org/wiki/Tube_(shell)

# 5.1 Questions

10/ C’est quoi un processus id ? Citez vos sources.

Un processus id est un code unique attribué sur les systèmes Unix ou Windows à tout processus lors de son démarrage. 
Il permet ainsi d'identifier le processus dans la plupart des commandes s'appliquant sur un processus donné (comme kill).

Source:https://fr.wikipedia.org/wiki/Identifiant_de_processus
