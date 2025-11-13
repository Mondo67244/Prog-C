
                                              //Exercice 1 : recherche sequentielle (ou lineaire)
                                                1 - ecrivons le pseudo code de la recherche sequentielle

                                                  Algorithme Recherche_Sequentielle
                                                  var x ,i : entier;
                                                  var tableau : Tableau [18, 9,23,41,7,15,4];
                                                  Debut 
                                                    Ecrire("Entrer la valeur a rechercher : ");
                                                    lire(x);
                                                    pour i allant de 0 a 6 faire
                                                      si Tableau[i] = x alors
                                                        Ecrire("Element trouve a la position ",i);
                                                        Retourner i
                                                      fin si
                                                    fin pour
                                                    Ecrire("Element non trouve dans le tableau");
                                                  Fin 

                                                  2 - L'algorithme renvoi :
                                                    --> 5 si x = 15 
                                                    --> "Element non trouve dans le tableau" si x = 50

                                                  3 - Comptons le nombre de comparaisons dans le pire des cas :
                                                    --> Dans le pire des cas on aura 7 comparaisons (n=7)
                                                      // Donc la complexite est O(n) avec n le nombre d'elements du tableau
                                                    --> Dans le meilleur des cas on aura 2 comparaisons
                                                      // Donc la complexite est O(2) dans le meilleur des cas

                                              //Exercice 2 : recherche dichotomique (ou binaire)
                                                1 - Ecrivons le pseudo code de la recherche dichotomique

                                                  Algorithme recherche_dichotomique
                                                  var tableau : Tableau [3,5,7,10,15,18,21,25,31]
                                                  var taille = 9, debut = 0, fin = taille - 1, position = -1, milieu, valeur;
                                                  Debut
                                                      Ecrire("Entrer le nombre a rechercher : ");
                                                      lire(valeur);
                                                      tantque (debut <= fin) faire
                                                          milieu = ( debut + fin )/2;
                                                              si (tableau[milieu] == valeur) alors
                                                                  position = milieu;
                                                                  fin = debut - 1
                                                              sinon
                                                              //Droite
                                                                  si (tableau[milieu] < valeur) alors
                                                                          debut = milieu + 1
                                                                  sinon 
                                                                  //Gauche
                                                                      si (tableau[milieu] > valeur) alors
                                                                          fin = milieu - 1
                                                                      finsi
                                                                  finsi
                                                              finsi
                                                      fintanque
                                                  Fin

                                                2 - Tracons les comparaisons 
                                                *Pour x = 15 :
                                                phase 1 : tableau[debut] = 3 , tableau[milieu] = 15 , tableau[fin] = 31 
                                                x = milieu = 15 , position = 4

                                                *Pour x = 22 :
                                                phase 1 : tableau[debut] = 3 , tableau[milieu] = 15 , tableau[fin] = 31  or 22 > tableau[milieu]
                                                >> phase 2 : tableau[debut] = 18 , tableau[milieu] = 21 , tableau[fin] = 31 or 22 > tableau[milieu]
                                                >>> phase 3 : tableau[debut] = 21 , tableau[milieu] = 25, tableau[fin] = 31 or 22 < tableau[milieu]
                                                >>>> phase 4 : tableau[debut] = 21, tableau[fin] = 31 , tableau[milieu] = 31 
                                                    Erreur , valeur 22 n'existe pas dans le tableau !!

                                                3 - Nombre maximun de comparaison pour un tableau de taille "n" est O(n)


                                              //Exercice 3 : Recherche par table de hashage


                                                1 - Calculons la position de chaque cle :
                                                * 15 : h(15) =  15 % 7 = 1
                                                * 8 : h(8) =  8 % 7 = 1
                                                * 22 : h(22) =  22 % 7 = 1
                                                * 29 : h(29) =  29 % 7 = 1
                                                * 14 : h(14) =  14 % 7 = 0
                                                * 7 : h(7) =  7 % 7 = 0

                                                2 - Montrons la table apres insertion avec chainage : 
                                                  Dans une liste chainee, les differents elements ayant la meme valeur de hashage se suivent.
                                                

                                                0  -> ("h(14)", pointeur) -> ("h(7)",pointeur) -> NULL
                                                1  -> ("h(15)", pointeur) -> ("h(8)",pointeur) -> ("h(22)",pointeur) -> ("h(29)",pointeur) -> NULL
                                                2  -> NULL
                                                3  -> NULL
                                                4  -> NULL
                                                5  -> NULL
                                                6  -> NULL

                                                3 - Indiquons le resultat de recherche des cles :

                                                *22 : 
                                                h(22) =  22 % 7 = 1
                                                1  -> ("h(15)",1) -> ("h(8)",1) -> ("h(22)",1) //Valeur 22 trouvee

                                                *30 :
                                                h(30) = 30 % 7 = 2
                                                la cle 2 n'existe pas dans le tableau (NULL)


                                            //Exercice 4 : Recherche par arbre binaire

                                                1 - Construisons un arbre binaire de recherche :

                                                            20 
                                                          /     \ 
                                                        10       30
                                                       /  \     /  \
                                                     5     15  25   40
                                                                    /
                                                                  35

                                                2 - Ecrivons le pseudo code de la recherche dans un arbre binaire de recherche

                                                Algorithme Recherche_Arbre_Binaire
                                                var racine : Pointeur_Arbre;
                                                var valeur : entier;
                                                Debut
                                                  Ecrire("Entrer la valeur a rechercher : ");
                                                  lire(valeur);
                                                  resultat <- Rechercher(racine, valeur);
                                                  si resultat = -1 alors
                                                    Ecrire("Element non trouve");
                                                  sinon
                                                    Ecrire("Element trouve : ", resultat);
                                                  finsi
                                                Fin

                                                Fonction Rechercher(racine : Pointeur_Arbre, valeur : entier) : entier
                                                Debut
                                                  si racine = NULL alors
                                                    Retourner -1
                                                  finsi
                                                  si racine.valeur = valeur alors
                                                    Retourner racine.valeur
                                                  sinon si valeur < racine.valeur alors
                                                    Retourner Rechercher(racine.gauche, valeur)
                                                  sinon
                                                    Retourner Rechercher(racine.droite, valeur)
                                                  finsi
                                                Fin

                                                3 - Montrons les appels recursifs pour x = 35
                                                rechercher(20,35)
                                                -> (35 > 20) -> appel recursif sur le "noeud Droit" rechercher(30,35)
                                                rechercher(30,35)
                                                -> (35 > 30) -> appel recursif sur le "noeud Droit" rechercher(40,35)
                                                rechercher(40,35)
                                                -> (40 > 35) -> appel recursif sur le "noeud Gauche" rechercher(35,35)
                                                rechercher(35,35)
                                                -> Element trouve !


                                            //Exercice 8 : Tri par fusion

                                            Exercice : Implementer en C

                                            Algorithme TriFusion

                                            Procedure TriFusion (A, gauche, droite)
                                              si (gauche >= droite) alors
                                                retourner -1
                                              finsi

                                              milieu <-- (gauche + droite)/2;
                                              TriFusion(A, gauche, milieu);
                                              TriFusion(A,milieu+1, droite);

                                              fusionner(A,gauche,milieu,droite)
                                            Finprocedure

                                            //Procedure pour fusionner les parties gauches et
                                            Procedure fusionner (A, gauche,milieu,droite)
                                              i <-- gauche
                                              j <-- droite
                                              k <-- 0

                                              Creer Tableau T de taille (droite -(gauche+1))
                                              
                                              tantque (i <= milieu Et j <= droite) faire
                                                si (A[i] <= A[j]) alors
                                                  T[k] <-- A[i]
                                                  i <-- i+1
                                                sinon
                                                  T[k] <-- A[j];
                                                  j <-- j+1;
                                                finsi
                                                k <-- k+1
                                              fintanque
                                            Finprocedure
                                          
                                              Debut
                                                ecrire("Entrez la taille du tableau : ");
                                                lire(N);

                                                //verifier la taille
                                                si (N <= 0) alors
                                                  ecrire("Taille invalide");
                                                  retourner -1;
                                                finsi

                                                Creer tableau A de taille N;

                                                //remplir le tableau
                                                pour (i de 0 a N-1) faire
                                                  Ecrire("Entrer l'element ", i, " :");
                                                  lire(A[i]);
                                                finpour

                                                Appeler TriFusion(A, 0, N-1);

                                                ecrire("Tableau trie : ");

                                                //afficher le tableau
                                                pour (i de 0 a N-1) faire
                                                  ecrire(A[i]);
                                                finpour
                                              Fin
