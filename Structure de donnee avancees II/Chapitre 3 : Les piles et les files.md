# Chapitre 3 : Les Piles et Les Files

Une file suit le principe **FIFO** : **First In First Out**.
Une pile suit le principe de **LIFO** : **Last In First Out**.

## I - Introduction

Les **Piles** et les **Files** sont des **structures de données linéaires** utilisées pour organiser les données. 

**Une File (queue)** est une structure de donnée basée sur le principe premier entré, premier sorti, en anglais **FIFO**. Cela veut dire que les premiers éléments ajoutés à la file seront les premiers à être récupérés ou retirés. Le fonctionnement ressemble à une file d'attente dans laquelle les premières personnes à arriver sont les premières à sortir de la file. 

Contrairement aux **Piles** qui suivent le principe **LIFO**, dans lequel les derniers éléments ajoutés à la pile seront les premiers à être retirés. Son fonctionnement ressemble à une pile d'assiettes où la dernière à être insérée sera la première à sortir.

## II - Opérations principales

### A) Les piles

Les opérations de base susceptibles d'être effectuées sur les **Piles** sont les suivantes : 

- `Empiler (PUSH)` : Ajouter un élément au sommet de la pile
- `Dépiler (POP)` : Retirer un élément au sommet de la pile
- `EstVide (isEmpty)` : Vérifier si une pile est vide
- `EstPleine (isFull)` : Vérifier si une pile est pleine
- `Sommet (TOP)` : Permet de consulter le sommet d'une pile
- `Taille (size)` : Retourne le nombre d'éléments de la pile

#### 1) Implémentation en pseudo-code

**Structure d'une pile :**
```java
Pile : tableau de taille MAX
Sommet : entier
```

**Initialisation d'une pile :** 
```java
InitialisationPile() {
    sommet <---- -1;
}
```

**Tester si pile vide :**
```java
EstVide() {
    retourner (sommet = -1);
}
```

**Tester si la pile est pleine :**
```java
EstPleine() {
    retourner (sommet = MAX - 1);
}
```

**Fonction Empiler :**
```java
Empiler(x) {
    si (EstPleine()) alors
        afficher("La pile est deja pleine !");
    sinon
        sommet <---- sommet + 1;
        pile[sommet] <---- x;
    finsi
}
```

**Fonction Dépiler :**
```java
Depiler(x) {
    si (EstVide()) alors
        afficher("La pile est vide !");
    sinon 
        x <---- pile[sommet];
        sommet <---- sommet - 1;
    finsi
}
```

**Fonction donnant le sommet :**
```java
Sommet(x) {
    si (EstVide()) alors
        afficher("La pile est vide !");
    sinon
        retourner (pile[sommet]);
    finsi
}
```

### B) Les files

Ce sont des structures de données fondamentales utilisées dans de nombreuses applications informatiques pour gérer les données de manière ordonnée et efficace.

Elles sont utilisées dans la gestion des processus en Système d'Exploitation, dans les files d'attente, les systèmes de messagerie, la gestion des impressions et la gestion des ressources partagées. 

#### 1) Fonctions ou Primitives

Voici les primitives communément utilisées pour manipuler les files :

- `Enfiler (enqueue)` : Ajoute un élément dans la file (à la fin).
- `Défiler (dequeue)` : Retire un élément de la file (au début) et renvoie le prochain à sa position.
- `EstVide` : Vérifie si la file est vide.
- `EstPleine` : Vérifie si la file est pleine.
- `Taille` : Retourne le nombre d'éléments dans la file.
- `Tête (front)` : Permet de consulter le premier élément.
- `Queue (rear)` : Permet de consulter le dernier élément.

**Structure d'une file :**
```java
Structure file
    file : tableau de taille MAX
    tete : entier
    queue : entier
FinStructure
```

**Initialisation d'une file :**
```java
Initialiser() {
    tete <---- 0;
    queue <---- -1;
}
```

**EXERCICE** : Écrire les primitives restantes (`vide`, `pleine`, `enfiler`, `defiler`, `taille`) en C.
