Une file suit le principe **<mark>FIFO</mark>**  :  **F**irst **I**n **F**irst **O**ut

Une pile suit le principe de **<mark>LIFO</mark>** : **L**ast **I**n **F**irst **O**ut

## I -**INTRODUCTION :**

les **Piles** et les **Files** sont des **<u>structures de donnees lineaires</u>** utilisees pour **Organiser les donnees**. 

 **Un File** (queue) : est une structure de donnee basee sur le principe premier entrer premier sorti, en anglais **FIFO**. Cela veut dire que<mark> les premiers elements ajoutes a la file </mark> seront les premier a etre **recuperes** ou **retires**. Le fonctionnement ressemble a une file d'attente dans laquelle les premieres personnes a arrriver sont les premieres a sortir de la file. 

Contrairement au **Piles** qui suivent le principe **LIFO** , dans lequel <mark>les derniers elements ajoutes a la pile</mark>  seront les premiers a etre retires. Sont fonctionnement ressemble a **une pile d'assiettes** ou la derniere  a etre inseree sera la premiere a sortir

## II - <u>Operations principales </u>:

### A) <u>Les piles </u>:

Les operations de base susceptibles d'etre effectuees sur les **Piles** sont les suivantes : 

- `Empiler (PUSH)` : Ajouter un element au sommet de la pile

- `Depiler (POP) `: Retire un element au sommet de la pile

- `EstVide (isEmpty)` : Verifier si une pile est vide

- `EstPleine (isFull)` : Verifier si une pile est pleine

- `Sommet (TOP) `: permet de consulter le sommet d'une pile

- `Taille (size)` : Retourne le nombre d'element de la pile

#### 1) Implementation en pseudo - code  :

***Structure d'une pile :**

    `Pile` : tableau de taille MAX

    `Sommet` : de type entier

***Initialisation d'une pile :** 

```java
InitialisationPile(){
    sommet <---- - 1;
}
```

***Tester si pile vide :**

```java
EstVide(){
    retourner(sommet = - 1);
}
```

***Tester si la pile est pleine :**

```java
EstPlein(){
    retourner(Sommet = Max-1);
}
```

***Fonction Empiler :**

```java
Empiler(x){
    si (EstPleine())
         afficher("La pile est deja pleine !");
        sinon
         sommet <---- sommet + 1;
         pile[sommet] <---- x;
    finsi
}
```

***Fonction Depiler :**

```java
Depiler(x){
    si (EstVide()) alors
        afficher("La pile est vide !");
       sinon 
        x <---- pile[sommet];
        sommet <---- sommet - 1;
    finsi
}
```

***Fonction donnant le sommet :**

```java
Sommet (x){
    si (EstVide()) alors
      afficher("La pile est vide !");
    sinon
      retourner(pile[sommet]);
    finsi
}
```

### B)  Les files :

Ce sont des structures de donnee fondamentales utilisees dasn de nombreuses applications informatiques pour gerer les donnees de maniere ordonees et efficaces.

Elles sont utilisees dans `la gestion des processus` en **SE** , dans les `files d'attente`, `systemes de messagerie`, `gestion des impressions` et `gestion des ressources partagees`. 

#### 1) Fonctions ou Primitives :

voici les primitives communement utilisees pour manipuler les files

- `Enfiler` (enqueue) : Ajoute un element dans la file (a la `fin`)

- `Defiler` (dequeue) : Retire un element de la file et renvoie le prochain a sa position (au `debut`).

- `EstVide` : verifie si la file est vide

- `EstPleine` : verifie si la file est pleine

- `Taille` : Retourne le nombre d'elements dans la file

- `tete` (front): Permet de consulter le premier element

- `Queue` (rear) : Permet de consulter le dernier element

***Structure d'une file :**

   `file` : tableau de taille `max`

        `tete` : entier

        `queue` : entier

    FinStructure

***Initialisation d'une file :**

    `initialiser(){ tete <---- 0; queue <---- -1;}`



**EXERCICE** : ECRIRE LES PRIMITIVES RESTANTES , `vide` , `pleine`, `enfiler`, `defiler` , `taille` en `C` .


