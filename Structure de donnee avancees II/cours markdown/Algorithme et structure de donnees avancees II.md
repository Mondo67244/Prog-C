# Algorithme et structure de données avancées II

## Chapitre 2 : Listes Chaînées

Une liste chaînée est une structure de donnée linéaire utilisée en programmation pour stocker une collection d'éléments. Contrairement aux tableaux qui sont stockés en mémoire de manière contiguë.

Une liste chaînée est composée d'une suite de blocs nommés **"noeuds"**, chacun contenant : 
- Une valeur (donnée) ;
- Un lien (pointeur) vers le noeud suivant.

Une liste chaînée est une structure linéaire qui n'a pas de dimension fixée à sa création, ses éléments de même type sont éparpillés dans la mémoire et reliés entre eux par des pointeurs. Sa dimension (taille) peut être modifiée selon la place disponible en mémoire. La liste est accessible uniquement par sa tête de liste (Premier Elément).

### 1) Liste simplement chaînée

- Chaque cellule (noeud) contient en plus des données, un pointeur vers l'élément suivant de la liste, le pointeur du dernier élément a pour valeur `NULL`.
- Une variable appelée "Premier" (tête) contient l'adresse du premier élément de la liste.
- Lorsque la liste est vide, la tête contient la valeur `NULL`.

Il existe des opérations de base susceptibles d'être manipulées sur une telle liste :
- Ajout d'un élément en tête de liste
- Ajout d'un élément en fin de liste
- Suppression d'un élément en tête
- Suppression en fin de la liste

*(Algorithmes pour ces opérations disponibles dans le fichier du Chapitre 2).*

### 2) Listes doublement chaînées

C'est une structure de donnée dynamique composée de noeuds (éléments) dans laquelle chacun d'eux contient : 
- Une valeur (information)
- Un pointeur vers le noeud précédent
- Un pointeur vers le noeud suivant

**NB :** Chaque élément est relié dans les deux sens.

Une liste doublement chaînée est repérée par les pointeurs `tête` et `queue` qui pointent respectivement sur le premier et le dernier élément de la liste. Le pointeur précédent du premier élément et le pointeur suivant du dernier élément contiennent la valeur `NULL`. Lorsque la liste est vide, `tête` et `queue` ont la valeur `NULL`.

#### a) Avantages et Inconvénients

**Avantages :**
- La liste peut être parcourue dans les deux sens.
- La suppression est plus facile par rapport à une liste simplement chaînée.
- L'insertion est plus flexible.

**Inconvénients :**
- Manipulation plus complexe.
- Consomme plus de mémoire car on a besoin de deux pointeurs (suivant et précédent) par noeud.

#### b) Les opérations de parcours

- **Parcours vers l'avant :** Il s'agit de parcourir la liste de la `tête` vers la `queue`.
```text
Algorithme ParcoursAvant(tete)
    Courant <--- tete;
    Tantque (courant != NULL) faire
        Afficher (courant.valeur);
        courant <--- courant.suivant;
    Fintantque
FinAlgorithme
```

- **Parcours vers l'arrière :** Il s'agit de parcourir la liste de la `queue` vers la `tête`.
```text
Algorithme ParcoursArriere(Queue)
    Courant <--- Queue;
    Tantque(courant != NULL) faire
        Afficher(Courant.valeur);
        Courant <--- courant.precedent;
    Fintantque
FinAlgorithme 
```

#### c) Les opérations d'insertion
- Insertion au début
- Insertion à la fin
- Insertion après un élément spécifique *(exercice)*

#### d) Les opérations de suppression

- **Suppression au début :**
```text
Algorithme SupprimerDebut(tete)
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

- **Suppression du dernier élément :**
```text
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

- **Suppression d'un élément donné :** *(exercice)*

#### e) Recherche dans la liste doublement chaînée

```text
Algorithme Recherche(tete, valeur)
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

### 3) Domaines d'application des listes chaînées

Les listes doublement chaînées sont utilisées dans les environnements suivants :
- Navigateurs web (boutons précédents et suivants)
- Les gestionnaires de musiques (suivant/précédent)
- Les systèmes d'historiques
- Les algorithmes de recherche dans le cache

### 4) Liste chaînée circulaire

#### a) Définitions

Une liste chaînée circulaire est une liste dans laquelle chaque élément (noeud) contient : 
- Une valeur
- Un pointeur vers le noeud suivant

**NB :** Le dernier élément pointe vers le premier, formant une boucle fermée.

Contrairement aux **listes simplement chaînées**, il n'y a pas de pointeur `NULL` à la fin, par conséquent on peut parcourir la liste **indéfiniment**. 

#### b) Opérations fondamentales

- **Insertion dans une liste circulaire vide :**
```text
Procedure InsererVide(tete, val)
    nouveau <---- CreerNoeud(val)
    nouveau.suivant <---- nouveau
FinProcedure
```

- **Insertion en tête :**
```text
Procedure InsererDebut(tete, val)
    nouveau <---- creernoeud(val)
    si tete == NULL alors
        nouveau.suivant <---- nouveau
        tete <---- nouveau
    sinon
        T <---- tete
        Tantque T.suivant != tete faire
            T <---- T.suivant
        FinTantque
        T.suivant <---- nouveau
        nouveau.suivant <---- tete
        tete <---- nouveau
    finsi
FinProcedure
```

- **Insertion en fin :**
```text
Procedure InsererFin(tete, val)
    nouveau <---- creernoeud(val)
    si tete == NULL alors
        nouveau.suivant <---- nouveau
        tete <---- nouveau
    sinon
        T <---- tete
        Tantque T.suivant != tete faire
            T <---- T.suivant
        FinTantque
        T.suivant <---- nouveau
        nouveau.suivant <---- tete
    finsi
FinProcedure
```

- **Parcours d'une liste circulaire :**
```text
Procedure parcours(tete)
    si (tete == NULL) alors
        afficher("Liste vide");
    finsi
    T <---- tete
    Repeter
        afficher(T.valeur);
        T <---- T.suivant
    Jusqu'a (T.suivant = tete) ou Jusqu'a (T != tete)
```

**EXERCICE :** Écrire la fonction de suppression en tête et en fin.
