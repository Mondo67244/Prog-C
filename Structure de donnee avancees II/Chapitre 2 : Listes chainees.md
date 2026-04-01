# Chapitre 2 : Listes Chaînées

## Introduction 
Une liste chaînée est une structure de données linéaire utilisée en programmation pour stocker une collection d'éléments, contrairement aux tableaux qui sont stockés en mémoire de manière contiguë.

Une liste chaînée est composée d'une suite de blocs nommés **"noeuds"**, chacun contenant : 
- Une valeur (donnée) ; 
- Un lien (pointeur) vers le noeud suivant.

Une liste chaînée est une structure linéaire qui n'a pas de dimension fixée à sa création, ses éléments de même type sont éparpillés dans la mémoire et reliés entre eux par des pointeurs. Sa dimension (taille) peut être modifiée selon la place disponible en mémoire. La liste est accessible uniquement par sa tête de liste (Premier élément).

## I - Intérêt des listes chaînées

On utilise les listes chaînées lorsque l'on doit traiter des objets représentés par suites sur lesquelles on doit effectuer de nombreuses suppressions et de nombreux ajouts.

### 1 - Avantages
- **Taille dynamique / flexible :** la taille de liste peut varier à l'exécution, pas besoin de savoir d'avance la taille de la structure ; 
- **Insertion / Suppression efficace :** ces opérations sont rapidement traitées dans la dite structure de donnée car on n'a pas besoin de déplacer les éléments contrairement aux tableaux.
- **Optimisation de la mémoire :** pas besoin d'allouer un bloc contigu de mémoire.

### 2 - Inconvénients
- **Accès séquentiel uniquement :** Impossible d'accéder directement à un élément par son indice comme les tableaux car il faut parcourir la liste séquentiellement.
- **Surcoût de la mémoire :** chaque noeud nécessite de l'espace supplémentaire pour stocker le pointeur.
 
## II - Les types de listes chaînées

Il existe différents types de listes chaînées, à savoir : 
- **Listes simplement chaînées :** constituées d'éléments reliés entre eux par des pointeurs dans un sens.
- **Listes doublement chaînées :** avec un parcours possible dans les deux sens.
- **Listes circulaires :** où le dernier élément pointe vers le premier.

Ces différents types peuvent être mixés selon les besoins.
     
### 1 - Listes simplement chaînées

*(voir liste-chainee.png)*

- Chaque cellule (noeud) contient en plus des données, un pointeur vers l'élément suivant de la liste, le pointeur du dernier élément a pour valeur `NULL`.
- Une variable appelée `"Premier"` (ou `tête`) contient l'adresse du premier élément de la liste.
- Lorsque la liste est vide, la tête contient la valeur `NULL`.
   
#### Opérations de base sur une liste simplement chaînée

##### Structure d'un Noeud
```text
structure Noeud
    valeur
    suivant
finstructure
```

##### 1. Ajout d'un élément en tête de liste
**Principe :**
- Etape 1 : Commencer par créer une nouvelle cellule et initialiser son champ "valeur".
- Etape 2 : Ajouter le nouvel élément en début de la liste en le faisant pointer vers l'ancien premier élément de la liste.
- Etape 3 : Mettre à jour la tête de la liste.

**Algorithme :**
```text
Algorithme InsertionEnTete(Liste, X)
    Nouveau <--- allouer Noeud; // Créer un nouveau noeud en memoire
    Nouveau.valeur <--- X;      // Stocker la valeur X
    Nouveau.suivant <--- Liste.tete; // Relier au premier noeud existant
    Liste.tete <--- Nouveau;    // Mettre a jour la tete
FinAlgorithme
```

##### 2. Ajout d'un élément en fin de liste
**Principe :**
- Etape 1 : Commencer par créer un nouveau noeud et initialiser son champ "valeur".
- Etape 2 : 
  - Si la liste est vide : le nouveau noeud devient la tête.
  - Sinon : se positionner sur le dernier élément de la liste et ajouter le nouveau noeud à sa suite.

**Algorithme :**
```text
Algorithme InsertionEnFin(Liste, X) 
    Nouveau <--- AllouerNoeud; 
    Nouveau.valeur <--- X; 
    Nouveau.suivant <--- NULL; // (fin de liste)
    
    Si (Liste.tete = NULL) Alors
        Liste.tete <--- Nouveau; 
    Sinon
        Courant <--- Liste.tete;
        Tantque (Courant.suivant != NULL) Faire 
            Courant <--- Courant.suivant; // Parcourt jusqu'au dernier
        Fintantque
        Courant.suivant <--- Nouveau; // Relie le dernier au nouveau
    Finsi
FinAlgorithme
```

##### 3. Suppression d'un élément en tête
**Principe :**
Il s'agit de supprimer le premier noeud, le nouveau premier élément devient alors le deuxième.

**Algorithme :**
```text
Algorithme SuppressionEnTete(Liste)
    Si (Liste.tete != NULL) Alors 
        Temp <--- Liste.tete; 
        Liste.tete <--- Liste.tete.suivant; 
        Liberer(Temp); 
    Sinon
        Afficher("La liste est deja vide, suppression impossible");
    Finsi
FinAlgorithme
```

##### 4. Suppression d'un élément après un noeud donné
**Principe :**
Ici on supprime le noeud qui suit un élément donné.

**Algorithme :**
```text
Algorithme SuppressionApresElement(Noeud)
    Si (Noeud != NULL et Noeud.suivant != NULL) Alors 
        Temp <--- Noeud.suivant; 
        Noeud.suivant <--- Noeud.suivant.suivant; 
        Liberer(Temp); 
    Sinon
        Afficher("Suppression impossible : Noeud inexistant ou dernier element");
    Finsi
FinAlgorithme
```

##### 5. Suppression en fin de la liste
**Principe :**
Il s'agit de supprimer le dernier noeud.

**Algorithme :**
```text
Algorithme SupprimerFin(tete)
    Si (tete = NULL) Alors
        Afficher("Liste vide");
    Finsi
    
    Si (tete.suivant = NULL) alors
        Liberer(tete);
        tete <--- NULL;
    Sinon
        Courant <--- tete;
        Tantque (Courant.suivant.suivant != NULL) Faire
            Courant <--- Courant.suivant;
        Fintantque
        Liberer(Courant.suivant);
        Courant.suivant <--- NULL; 
    Finsi
FinAlgorithme
```
