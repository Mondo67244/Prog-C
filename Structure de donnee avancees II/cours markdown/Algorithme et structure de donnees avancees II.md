# Algorithme et structure de donnees avancees II

## Chapitre 2: Listes chainnees

Une liste chainnees est une structure de donnee lineaire utilisee en programmation pour stocker une collection d'elements. Contrairement aux tableaux qui sont stockees en memoire de maniere contigue.

Une liste chainnees est composee d'une liste de blocs nommes "noeuds", chacun contenant : 

- Une valeur (donnee);

- Un lien (pointeur) vers le noeud suivant;

Une liste chainnee est une structure lineaire qui n'as de dimension fixee a sa creation, ses elements

|

|

|

**Suppression en fin de la liste :**

Ici il s'agit de supprimer le noeud, l'algorithme est le suivant :

```java
Algorithme SupprimerFin(tete)
   Si (tete = NULL) Alors
        Afficher("Liste vide");
   Finsi
   Si (tete.suivant = NULL) alors
         liberer(tete)
         tete <--- NULL;
```

## 2) Listes doublement chainnees :

C'est une structure de donnee dynamique composee de noeuds (elements) dans le lequel chacun d'eux (noeuds) contient : 

- Une valeur (information)

- Un pointeur vers le noeud precedent

- Un pointeur vers le noeud suivant

**NB:** Chaque element est relie dans les deux sens

 **Figure illustrative :**

Une liste doublement chainnee est reperee par les pointeurs tete et queue qui pointent respectivement sur le premier et le dernier element de la liste.

Le pointeur precedent du premier element et le pointeur suivant du dernier element contiennent la valeur `NULL`. Lorsque la liste est vide, tete et queue ont la valeur  `NULL`,

#### a) Avantages et Inconvenients :

**Avantages :**

- La liste est parcourue dans les deux sens

- La suppression est plus facile par rapport a une liste simplement chainee

- L'insertion est plus flexible

**Inconvenients :**

- Manipulation plus complexe

- Consomme plus de memoire car on a besoin de deux pointeurs (suivant et precedent)

#### b) Les differentes operations susceptibles d'etres effectuees :

- Parcours vers l'avant : il s'agit de parcourir la liste de la `tete` vers la `queue`. L'algorithme est le suivant : 

```java
Algorithme ParcoursAvant(tete)
     Courant <--- tete;
    Tantque (courant != NULL) faire
        Afficher (courant.valeur);
        courant <--- courant.suivant;
    Fintantque
FinAlgorithme
```

- Parcours vers l'arriere : il s'agit de parcourir la liste de la queue vers la tete. L'algorithme est le suivant : 

```java
Algorithme ParcoursArriere(Queue)
     Courant <--- Queue;
    Tantque(courant != NULL) faire
        Afficher(Courant.valeur);
        Courant <--- courant.precedent;
    Fintantque
FinAlgorithme 
```

#### c) Les operations d'insertion  :

- Insertion au debut : Algo

- Insertion a la fin :

- Insertion apres un element specifique (exercice) :

#### d) Les operations de suppression :

- Suppression au debut : son algorithme est le suivant :

```java
ALgorithme SupprimerDebut(tete)
    Si (tete = NULL) alors
        Afficher("Erreur");
    Finsi
      T <--- tete;
      tete <--- tete.suivant;
    Si (tete != NULL) alors
      tete.precedent <--- NULL;
    Finsi
    Liberer(T);
FinAlgorithme
```

- Suppression du dernier element :  l'algoritme est le suivant  :

```java
Algorithme SupprimerFin(tete)
    Si (tete = NULL) alors
        Afficher("Erreur");
    Finsi
    Courant <--- tete;
    Tantque (courant.suivant != NULL) alors
        Courant <--- Courant.suivant;
    Fintantque
    Si (Courant.suivant != NULL) alors
        Courant.precedent.suivant <--- NULL;
    Sinon
        tete <--- NULL;
    Finsi
    Liberer (Courant);
FinAlgorithme
```

- Suppression d'un element donne (exercice) : 

#### e) Recherche dans la liste doublement chainnee :

L'algorithme est le suivant :

```java
Algorithme Recherche (tete, valeur)
    Courant <--- tete;
  Tantque (Courant != NULL) faire
       Si (Courant.valeur = valeur) alors
           retourner Courant;
       Finsi
    Courant <--- Courant.suivant;
  Fintantque
 retourner NULL;
FinAlgorithme
```

## 3) Domaines d'application des listes chainees :

Les listes doublement chainees sont utilisees dans les environnements suivants :

- Navigateurs web (boutons precedents et suivants)

- Les gestionnaires de musiques

- Les systemes d'historiques

- Les algorithmes de recherche dans le cache

## 4) Liste chainee circulaire :

## a) definitions :

Une liste chainee circulaire est une liste dans laquelle chaque elements (Noeud) contient : 

- Une valeur

- Un pointeur vers le noeud suivant

**NB:**  le dernier element pointe vers le premier . formant une boucle

Contrairement aux **listes simplement chainee** il n'y a pas de pointeur **NULL** a la fin, par consequent on peut parcourir la **liste indefiniment** . 

## b) Quelques Operations fondamentales:

- **Insertion dans une liste circulaire vide :** le pseudo code est le suivant

```java
Procedure Inserervide(tete, val)
    nouveau <---- CreerNoeud(val)
    nouveau.suivant <---- nouveau
FinProcedure
```

- **Insertion en tete :** le pseudo code est le suivant: 

```java
Procedure InsererDebut(tete, val)
    nouveau <---- creernoeud(val)
    si tete == NUL alors
        nouveau.suivant <---- nouveau
            tete <---- nouveau
        sinon
            T <---- tete
    Tantque T.suivant != tete faire
            T <---- T.suivant
    FinTanque
        T.suivant <---- nouveau
        nouveau.suivant <---- tete
            tete <---- nouveau
    finsi
FinProcedure
```

- **Insertion en fin :** le pseudo code est le suivant

```java
Procedure InsererFin(tete, val)
    nouveau <---- creernoeud(val)
    si tete == NUL alors
        nouveau.suivant <---- nouveau
            tete <---- nouveau
        sinon
            T <---- tete
    Tantque T.suivant != tete faire
            T <---- T.suivant
    FinTanque
        T.suivant <---- nouveau
        nouveau.suivant <---- tete
    finsi
FinProcedure
```

- **Parcours d'une liste circulaire :** la procedure est la suivante

```java
Procedure parcours (tete)
    si (tete = NUL) alors
        afficher("Liste vide");
    finsi
    T <---- tete
    Repeter
        afficher(T.valeur);
            T <---- T.suivant
    Jusqu'a(T.suivant = tete) ou Jusqu'a(T != tete)
```

EXERCICE : Ecrire la fonction de suppression en tete et en fin
