//Exercice 1 :
On dispose d'une table de hashage de taille M = 10, la fonction de hashage est h(k) = k Mod 10. 
Inserer successivement les cles suivantes : 25, 37, 48, 59, 62, 73, 84 en utilisant :
 1 - le hashage ouvert (liste chainee)
 2 - le hashage lineaire (en cas de colision, prendre (h(k) + i ) ) Mod M
 3 - dessinner la table finale dans les deux cas
 4 - indiquer le nombre moyen de comparaison pour chaque cle inserees

 Solution :

 1 - inserons successivement :
   h(25) = 25 Mod 10 -> 5
   h(37) = 37 Mod 10 -> 7
   h(48) = 48 Mod 10 -> 8
   h(59) = 59 Mod 10 -> 9
   h(62) = 62 Mod 10 -> 2
   h(73) = 73 Mod 10 -> 3
   h(84) = 84 Mod 10 -> 4
   
    *Chainons :
    0 ->  NULL
    1 ->  NULL
    2 ->  ("h(62)", pointeur) -> NULL
    3 ->  ("h(73)", pointeur) -> NULL
    4 ->  ("h(84)", pointeur) -> NULL
    5 ->  ("h(25)", pointeur) -> NULL
    6 ->  NULL
    7 ->  ("h(37)", pointeur) -> NULL
    8 ->  ("h(48)", pointeur) -> NULL
    9 ->  ("h(59)", pointeur) -> NULL
      Donc il n'y a pas de Collision

  2 - Il n'y a pa de Collision
  3 - Table finale :
     
    * Liste chainnee : 
    -------------------------------------------------------------------------------
    | 0 |  1 |   2   |    3   |    4   |   5    |   6   |   7    |   8   |    9   |
    -------------------------------------------------------------------------------
    | 0 |  0 |(62,0) | (73,0) | (84,0) | (25,0) | (0,0) | (37,0) |(48,0) | (59,0) |
    -------------------------------------------------------------------------------

    * Hashage lineaire :

    -----------------------------------------------------------------------------------------------------------------------------------
    |        Cle         |   0  |  0  |     62    |    73     |    84     |     25    |     0     |    37     |    48     |    59     |
    -----------------------------------------------------------------------------------------------------------------------------------
    |        H(k)        |   0  |  1  |     2     |     3     |     4     |     5     |     6     |     7     |     8     |     9     |
    -----------------------------------------------------------------------------------------------------------------------------------
    | Table d'insertion  |   0  |  0  | T[2] = 62 | T[3] = 73 | T[4] = 84 | T[5] = 35 |     0     | T[7] = 37 | T[8] = 48 | T[9] = 59 |
    -----------------------------------------------------------------------------------------------------------------------------------
    | Nombre comparaison |                                               1                                                            |
    -----------------------------------------------------------------------------------------------------------------------------------

     -> Tableau final  :
    --------------------------------------------------------------------------
    | Adresses | 0 |  1 |  2  |  3   |  4   |  5   |  6  |  7   |  8  |   9  |
    --------------------------------------------------------------------------
    | Valeur   | 0 |  0 |(62) | (73) | (84) | (25) | (0) | (37) |(48) | (59) |
    --------------------------------------------------------------------------

  4 - nombre moyen de comparaison pour chaque cles : 
    h(62) = 1 comparaison
    h(73) = 1 comparaison
    h(84) = 1 comparaison
    h(25) = 1 comparaison
    h(37) = 1 comparaison
    h(48) = 1 comparaison
    h(59) = 1 comparaison



//Exercice 2 :
En exploitant la table de l'Exercice precedent : 
 1 - Rechercher les cles 37, 84, 99
 2 - Indiquer le nombre de comparaisons necessaires dans chaque methode de gestion des colisions

 Solution :

 1 - Recherchons pour chaque cle :
    h(37) = 37 Mod 10 = 7
    h(84) = 84 Mod 10 = 4
    h(99) = 99 Mod 10 = 9 , il y a Collision avec la cle h(59)  on aura donc :
    9 ->  ("h(59)", pointeur) -> ("h(99)", pointeur) -> NULL
 2 - Indiquons le nombre de comparaison necessaires pour chaque cle :
   pour : 
    h(37) = 1 comparaison
    h(84) = 1 comparaison
    h(99) = 2 comparaisons car 



//Exercice 3 : Construction d'un arbe binaire de recherche
1 - Constuire un arbre binaire de recherche a partir de la sequence suivante :
50, 30, 70, 20, 40, 60 et 80
2 - Rechercher les valeurs 60 et 35 puis montre le chemin suivi

 Solution :

 1 - Construisons l'arbre binaire de recherche : 

                    50
                  /    \
                30     70
               /  \    /  \ 
              20  40  60  80

 2 - Recherchons les valeurs 60 et 35 :
 soit les valeurs a chercher val1 = 60 et val2 = 35

  * Pour val1 :
recherche(50,val1) val1 > 50  donc on cherche sur le noeud Droit 70
recherche(70,val1) val1 < 50  donc on cherche sur le noeud Gauche 60
recherche(60,val1) val1 = 60  la valeur 60 a ete trouvee au noeud 60.

  * Pour val2 :
recherche(50,val2) val2 < 50  donc on cherche sur le noeud Gauche 30
recherche(30,val2) val2 > 30  donc on cherche sur le noeud Droit 40
recherche(40,val2) val2 < 40 , il n'y a plus de noeuds donc la valeur n'existe pas dans l'arbre


         

//Exercice 4 : Representation d'un graphe
Considerons un graphe non oriente a 05 sommets

        A - B ; A - C ; B - D ; C - D et C - E ; D - E

 1 - Representer ce graphe sous forme de matrice d'adjacence
          
                  A   B   C   D   E

                A  0   1   1   0   0

                B  1   0   0   1   0

                C  1   0   0   1   1

                D  0   1   1   0   1

                E  0   0   1   1   0

 2 - Representer le sous forme de liste d'adjacence

                  A : B , C , 
                  B : A , D
                  C : A , D , E
                  D : B , C , E
                  E : C , D

 //Exercice 5 : 
 soit le graphe oriente suivant :

            A ---- B
            | \
            |  \
            |   \
            |    \
            |     \ 
            C      D
              \   /
                E

  1 - Determiner le degre de chaque sommets
  2 - Etablir la liste d'adjacence
  3 - Etablir la matrice d'adjacence

    Solution :

  1 - Donnons les degres des sommets :

          A est de degre 3
          B est de degre 1
          C est de degre 2
          D est de degre 2 
          E est de degre 2

  2 - Etablissons la liste d'adjacence :

            A : B , C , D 
            B : A 
            C : A , E 
            D : A , E 
            E : C , D

  3 - Etablissons la matrice d'adjacence :

