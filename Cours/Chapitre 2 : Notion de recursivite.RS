                            CHAPITRE II : Notion de recursivite

                                I - Introduction:

                    La recursivite est une technique de programmation dans laquelle une fonction s'appelle elle meme , directement ou indirectement
                    pour resoudre un probleme precis. Elle permet de resoudre des problemes en les decomposant en sous problemes plus simples.
                    Elle permet egalement de gerer les structures de donnees recursives (Arbre/Graphe/Repertoires), elle est souvent utilisee dans les
                    structures de donnees comme :
                        - Les algorithmes de tri;
                        - Le calcul des nombres/suites de fibonacci;
                        - Le calcul du factoriel etc..

                                II - Types de recursivite :
                                    1 - structure d'une fonction recursive :

                    Traditionnellement une fonction recursive contient toujours :

                        - Le cas de base (condition d'arret) permettant d'arreter l'appel recursif :
                                                            if (nombre == 0 || nombre == 1){ // cas de base
                                                                return 1;
                                                            } else {
                                                                return nombre * factorecursif1(nombre -1);
                                                            }
                        - Appel recursif : la fonction s'appelle avec un sous probleme plus simple


                                    2 - recursivite directe :

                            C'est un type de recursivite dans laquelle une fonction s'appelle elle-meme directement dans son propre corps. Cela signifie que 
                            la fonction fait reference a elle meme directement ou implicitement lors de son execution. C'est egalementle type de recursivite
                            le plus courant et le plus simple a assimiler.
                                Voici un exemple de recursivite directe qui utilise une fonction en C pour calculer le factoriel d'un nombre:

                                        #include <stdio.h>
                                        int factorecursif1(int nombre){
                                        if (nombre == 0 || nombre == 1){
                                            return 1;
                                        } else {
                                            return nombre * factorecursif1(nombre -1);
                                        }
                                        }
                                        int main(){
                                            factorecursif1(5);
                                        }
                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
                            Dans cette exemple , la fonction factorecursif1 est recursive directe s'appelant elle meme directement 
                            dans son propre corps. Lorsqu'on appelle factorecursif1(5), la fonction s'appelle elle meme avec des valeurs
                            decroissantes de "nombre" jusqu'au cas de base (Qui met fin a la recursivite).

                            Remarque: la recursivite directe est souvent utilisee pour resoudre des problemes pouvant etre decompose en problemes
                            similaires au probleme initial en plus petits pour resoudre le meme probleme. Cependant il faut correctement definir 
                            un cas de base pour eviter d'appeller a l'infini et de s'assurer que chaque appel recursif se rapproche du cas de base.

                                    3 - recursivite indirecte :
                            C'est le type de recursivite dans laquelle une fonction appelle une autre qui l'appelle en retour.
                            * illustration : f --> g --> f
                            * Exemple : 
                                    //Declaration
                                    void f1 (int n);
                                    void f2 (int n);

                                    //Definition
                                    void f1 (int n){
                                        if (n > 0){
                                            printf("f1:%d",n);
                                            f2(n - 1);
                                        }
                                    }
                                    void f2 (int n){
                                        if (n > 0){
                                            printf("f2:%d",n);
                                            f1(n/2);
                                        }
                                    }

                            * Exercice : Implementer un programme en C qui demande un nombre entier a l'utilisateur puis teste et affiche sa parite
                            (via la recursivite indirecte) (TP1/exoCours.c)

                                    4 - Recursivite mutuelle ou croisee :
                            Elle se produit lorsque deux ou plusieurs fonctions s'appellent mutuellement de maniere recursive. ie Une fonction appelle une autre 
                            qui a son tour peut appeler la premiere et ainsi de suite, cela peut sembler complexe mais peut egalement etre une technique puissante
                            pour resoudre des problemes complexes

                                III - Evaluation d'un appel recursif :

                            Elle se deroule de maniere sequentielle, en ajoutant chaque appels a la pile des appels jusqu'a ce que le cas de base soit atteint.

                            * Exemple: 
                            
                            facto(4) = n * facto(4 - 1)
                            ---> 4! = 4 * 3!
                                -->  =  4 * 3 * 2!
                                -->  =  4 * 3 * 2 * 1!
                                --> =  4 * 3 * 2 * 0! (Cas de base atteint !)
                                resultat : 4! = 4 * 3 * 2 * 1


                            Une fois que le cas de base est atteint, les valeur sont envoyees de maniere recursives sortant ainsi de la pile des appels.
                            Voici comment cela fonctionne etape par etapes :

                                1 - l'appel initial :
                            lorsqu'une fonction recursive est appelle pour la premiere fois, son execution commence directement. Elle entre dans la fonction et 
                            rencontre la declaration recursive.

                                2 - Les appels recursifs:
                            La fonction s'appelle elle (recursivite directe) meme avec des arguments (parametres) modifies. Ces nouveaux appels recursifs sont 
                            empiles sur la pile des appels.

                                3 - Cas de base :
                            A un moment donne, la fonction atteint un cas de base (limite). C'est une condition speciale qui met fin a la recursion. Si le cas
                            de base est atteint, la fonction renvoie une valeur sans effectuer d'autres appels recursifs.
                                
                                4 - Retour des valeurs:
                            Une fois que le cas de base est atteint, les valeurs sont renvoyees en remontant la pile des appels. Chaque appels recursifs renvoie
                            une valeur a l'appel precedent qui peut alors continuer son execution (pour evaluer la valeur du retour).

                                5 - Desempilement des appels:
                            Les appels sont desenregistres de la pile des appels au fur et a mesure que les valeurs sont renvoyees jusqu'a ce que l'appel initial
                            soit atteint et que la fonction se termine.
                            
                            Il est essentiel de definir un cas de base correctement pour eviter une recursion infinie. Si le cas de base n'est pas 
                            correctement defini, la fonction continuera a s'appeler elle meme 'indefiniment', provocant ainsi un depassement de pile des appels
                            et eventuellement le crash du programme.
   

