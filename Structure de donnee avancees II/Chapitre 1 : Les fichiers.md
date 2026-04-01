# Chapitre 1 : Les Fichiers

## Introduction 
Un fichier est un ensemble structuré d'informations stockées sur un support de stockage (Disques Dur, Clés Usb, Cloud...). Il permet de conserver les données de manière organisée afin de pouvoir les consulter, les modifier ou les transmettre ultérieurement.
    
L'un des rôles essentiels d'un fichier c'est d'assurer la persistance des données (reste stockée même après la fermeture d'un programme). 
- **Une donnée persistante** : est celle qui continue d'exister même après la fermeture d'un programme ou l'arrêt d'un ordinateur (ex. Clé Usb), contrairement aux données stockées temporairement en mémoire vive qui disparaissent lorsque le système s'éteint.

> "Les fichiers constituent donc un mécanisme fondamental de stockage permanent."

Comme exemple on a : 
- Les documents textes enregistrés
- Une image sauvegardée
- Une base de donnée

## I - Définition et déclaration d'un fichier

Un fichier est un ensemble structuré de données sauvegardées afin d'être conservé de manière persistante. Il permet : 
- Stocker les informations de façon durable
- D'organiser les données
- De les lire, les modifier ou les réutiliser ultérieurement.

Un fichier possède donc généralement les éléments suivants : 
- **Un Nom** : pour identifier le fichier parmi d'autres
- **Une extension** : pour spécifier le type d'éditeur
- **Un emplacement (Le Chemin d'Accès)** : pour localiser l'emplacement de stockage
- **Un type de données** (texte, binaire, image, vidéo...) 

### 1 - Déclaration
En programmation, la déclaration d'un fichier consiste à créer une nouvelle variable qui va permettre de manipuler ce fichier (lecture, écriture, modification).
On déclare une variable de type fichier via la syntaxe suivante :

```java
TypeFichier nomFichier
```

Avant d'utiliser un fichier on doit au préalable effectuer les opérations suivantes : 
1. La déclarer
2. L'ouvrir
3. L'utiliser (lire / écrire)
4. La fermer (pour éviter les fuites de mémoire)

> **Remarque :** l'ouverture d'un fichier nécessite trois modes spécifiques à savoir : La lecture, l'écriture et l'ajout (modification).

**Exemple d'ouverture d'un fichier en langage C :** *(Légende : r = read, w = write, a = add)*

```c
FILE *fichier; // Déclaration
fichier = fopen("nomfichier.extension", "r"); // Ouvrir le fichier en précisant le mode, ici lecture
```

## II - Les opérations de base sur les fichiers

Plusieurs opérations sont essentielles lors de la manipulation des fichiers, et les plus fondamentales sont : 

### 1 - La déclaration d'un fichier
Elle permet de définir une variable de type fichier, la syntaxe est la suivante :

```java
// En algorithmique
Déclarer F:fichier
```

### 2 - L'ouverture d'un fichier
⚠️ On doit toujours ouvrir un fichier avant toute manipulation. La primitive utilisée est `Ouvrir` (`fopen` en C). On va distinguer trois modes d'ouverture :

```java
// En algorithmique
// Ouverture en Lecture : 
Ouvrir(F, "Lecture");

// Ouverture en Ecriture : 
Ouvrir(F, "Ecriture");

// Ouverture en Ajout : 
Ouvrir(F, "Ajout");
```

### 3 - Lecture dans un fichier
Elle permet de récupérer les données stockées dans un fichier pour des fins utiles. La primitive est `Lire` (`fread` en C). On va distinguer deux variantes :

```java
// Lecture d'un element : 
Lire(F, variable);

// Lecture jusqu'a la fin du fichier : 
Tantque nonFin(F) faire
    Lire(F, variable);
    Lire(variable);
Fintantque
```

### 4 - Écriture dans un fichier
Elle permet d'enregistrer les données dans le fichier, la primitive est `Ecrire` (`fwrite` en C), la syntaxe est : 

```java
// En algorithmique
Ecrire(F, donnee); // ex: Ecrire(F, "Bonjour");
```

### 5 - Ajout dans un fichier
Cette opération est similaire à celle de l'écriture mais elle s'effectue sans effacer le contenu existant. La syntaxe est : 

```java
Ecrire(F, "Ajout")
```

### 6 - Fermeture d'un fichier
Cette opération est très cruciale car elle permet de libérer les ressources et d'assurer une sauvegarde correcte des données (`fclose` en C). La syntaxe est : 

```java
Fermer(F);
```

### Exemple et Exercices

**Exemple :** Écrire un algorithme permettant d'écrire des nombres dans un fichier puis de les lire :

```java
Algorithme manipulationFichier
    Declarer F:fichier
    Declarer X:entier   
Debut
    // Ouverture du fichier en ecriture
    Ouvrir(F, "Ecriture");
    Pour X <-- 1 a 5 faire
        Ecrire(F, X);
    FinPour
    Fermer(F); // Remarque: seulement F

    // Ouverture du fichier en lecture
    Ouvrir(F, "Lecture")
    Tantque (nonFin(F)) faire
        Lire(F, X);
        Ecrire(X); // pour afficher
    Fintantque
    Fermer(F);
Fin
```

#### Exercices
- **Exercice 1 :** Écrire un algorithme qui permet de saisir 05 entiers puis de les enregistrer dans un fichier.
- **Exercice 2 :** Écrire un algorithme qui lit tous les nombres du fichier précédent et les affiche à l'écran.
- **Exercice 3 :** Écrire un algorithme qui calcule la somme des nombres stockés dans un fichier.
- **Exercice 4 :** Écrire un algorithme qui copie le contenu d'un fichier source vers un fichier destination.
- **Exercice 5 :** Écrire un algorithme qui compte le nombre d'éléments contenus dans un fichier.

#### Solution Commune

```java
Algorithme exercicesFichiers
    Declarer F, C, R: fichier
    Declarer X, i, r, val, compt: entier
Debut
    // EXO 1 : saisie de 05 entiers 
    Ouvrir(F, "Ecriture");
    Pour i allant de 1 a 5 Faire
        Ecrire("Entrer le nombre ", i, " :")
        Lire(X)
        Ecrire(F, X) // écrire dans le fichier
    FinPour
    Fermer(F);
    
    // EXO 2 : lire tous les nombres du fichier precedent
    Ouvrir(F, "Lecture");
    Tantque (nonFin(F)) Faire
        Lire(F, X)
        Ecrire(X) // afficher
    Fintantque
    Fermer(F);
    
    // EXO 3 : calcul de la somme des nombres dans le fichier
    r <-- 0;
    Ouvrir(F, "Lecture");
    Tantque (nonFin(F)) Faire
        Lire(F, X);
        r <-- r + X;
    Fintantque
    Fermer(F);
    
    // EXO 4 : Copie du contenu d'un fichier dans un autre
    Ouvrir(F, "Lecture");
    Ouvrir(R, "Ecriture");
    Tantque (nonFin(F)) Faire
        Lire(F, X);
        Ecrire(R, X); // ecrire dans le fichier destination
    Fintantque
    Fermer(F);
    Fermer(R);
    
    // EXO 5 : Comptage dans le fichier
    compt <-- 0;
    Ouvrir(F, "Lecture");
    Tantque (nonFin(F)) Faire
        Lire(F, X);
        compt <-- compt + 1;
    Fintantque
    Fermer(F);
Fin
```

## III - Implémentation en langage C

Le langage C offre un ensemble de fonctions standards pour manipuler les fichiers dont les principales utilisées sont : 

- `fopen()` pour l'ouverture
- `fclose()` pour la fermeture
- `fread()` pour la lecture
- `fwrite()` pour l'écriture
- `perror()` pour afficher un message d'erreur
- `fseek()` pour déplacer le pointeur du fichier à une position spécifiée
- `ftell()` pour retourner la position actuelle du pointeur de fichier

### 1 - La gestion des erreurs
Il est important de contrôler les erreurs lors de l'ouverture (`fopen()`), de la lecture (`fread()`) et de l'écriture (`fwrite()`) des fichiers. Cela se fait souvent en vérifiant si les pointeurs de fichiers sont à NULL :

```c
if (fichier == NULL) {
    perror("Erreur lors de l'ouverture du fichier");
    return 1;
}
```

### 2 - Test de la fin du fichier
Pour tester si la fin du fichier est atteinte on va utiliser la constante `EOF` (End Of File) ou manipuler la lecture de chaîne. La syntaxe pour un parcours caractère par caractère est la suivante : 

```c
char c; 
// fgetc permet de recuperer un caractère
while ((c = fgetc(fichier)) != EOF) { 
    // Traitement a effectuer
}
```

Pour lire des lignes, on utilise plutôt `fgets` :
```c
char ligne[256];
while (fgets(ligne, sizeof(ligne), fichier) != NULL) {
    // Traitement de la ligne
}
```

### Exercices d'implémentation en C
- **Exercice 2 :** Le programme C suivant crée un fichier `examen.txt`, puis il écrit quelques lignes de javae.
- **Exercice 3 :** Implémenter un programme C qui lit le contenu de `examen.txt` et l'affiche dans la console, utiliser `fgets` et déterminer la taille.
- **Exercice 4 :** Implémenter un programme C qui génère une séquence de nombres aléatoires entre 1 et 100, les écrit dans un fichier nommé `console.txt`, puis lit le fichier pour afficher les nombres à l'écran.
- **Exercice 5 :** Implémenter un programme C qui supprime le fichier `examen.txt` créé précédemment.
- **Exercice 6 :** Implémenter un programme C qui copie le contenu d'un fichier source nommé `source.txt` dans un fichier nommé `destination.txt`.