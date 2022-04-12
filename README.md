# Trello

## Autorzy
- Łukasz Orlikowski (gr 9, @lorlikowski)

## Opis
Celem zadania byłoby stworzenie aplikacji WWW służącej optymalizacji, zarządzaniu i planowaniu pracy własnej i zespołu à la Trello z różnymi rozwinięciami. Backend zostałby napisany w Ruście (najpewniej za pomocą frameworku Rocket). Dotego zostałby dołączony front (najpewniej framework yew) oraz testy.
## Funkcjonalność
- Tworzenie tablic (prywatnych i zespołowych) oraz tworzenie i zarządzanie zespołem
- Tworzenie list na tablicy (TODO, Running, itp.)
- Dodawanie/usuwanie/modyfikacja zadań oraz ustawianie deadlinu, przenoszenie pomiędzy listami
- Dodatkowe dane zadań: notatki, miejsce, lista osób przypisanych, lista zadań, możliwość przypisania osoby do konkretnego zadania z listy
- Log zdarzeń na liście (zmiana deadlinu, tytułu, dodanie osoby)
- Każdy użytkownik może startować swoje różne timery (np. do pomiaru czasu pracy)
- Historia timerów
- Tagowanie zadań i punkty funkcyjne
- Filtracja zadań po osobach. tagach, deadlinach, punktach funkcyjnych
- Dodawanie milestonów

## Propozycja podziału na części
- pierwsza część:
  -Tworzenie tablic (prywatnych i zespołowych) oraz tworzenie i zarządzanie zespołem
  - Tworzenie list na tablicy (TODO, Running, itp.)
  - Dodawanie/usuwanie/modyfikacja zadań oraz ustawianie deadlinu, przenoszenie pomiędzy listami
  - Dodatkowe dane zadań: notatki, miejsce, lista osób przypisanych, lista zadań, możliwość przypisania osoby do konkretnego zadania z listy

- druga część:
  - Log zdarzeń na liście (zmiana deadlinu, tytułu, dodanie osoby)
  - Każdy użytkownik może startować swoje różne timery (np. do pomiaru czasu pracy)
  - Historia timerów
  - Tagowanie zadań i punkty funkcyjne
  - Filtracja zadań po osobach. tagach, deadlinach, punktach funkcyjnych
  - Dodawanie milestonów

## Biblioteki
- Rocket (api)
- yew (frontend)
- Diesel (ORM do łączenia się z bazą danych (postgres) https://diesel.rs/)
