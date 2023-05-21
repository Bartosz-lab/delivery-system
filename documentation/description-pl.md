# System Dostaw

Celem tej aplikacji jest usprawnienie procesu zarządzania przesyłkami.

## Główne Funkcje

* ### Funkcje Kontrahenta
    * #### Rejestracja przesyłki
        Aplikacja umożliwia rejestrację nowych przesyłek w systemie. Przy rejestracji podaje się dane oraz adres odbiorcy, dane przesyłki takie jak wielkość oraz waga przesyłki, dane predefiniowanego magazynu nadawczego oraz datę odbioru przesyłki.
    * #### Śledzenie statusu przesyłki
        Aplikacja umożliwia śledzenie statusu przesyłki od jej zarejestrowania w systemie do momentu dostarczenia przesyłki do odbiorcy.
    * #### Raporty dotyczące rozliczeń
        Aplikacja umożliwia wygenerowanie raportów dotyczących kosztów poniesionych przez kontrahenta w związku z nadanymi przesyłkami. Dostępne są raporty dziennie, tygodniowo oraz miesięcznie, a także możliwość podziału na magazyny nadawcze.

* ### Funkcje pracownika firmy kurierskiej
    * #### Rejestracja nowego statusu przesyłki
        Aplikacja umożliwia rejestrację nowego statusu oraz lokalizacji przesyłki. Umożliwia to śledzenie trasy, jaką przebyła przesyłka, oraz automatyczne wysłanie odpowiednich powiadomień do kontrahenta i/lub odbiorcy przypisanego do konkretnych statusów.
    * #### Raporty
        Aplikacja umożliwia generowanie raportów:
        * Informacje o przesyłkach do odebrania z danych magazynów nadawczych.
        * Informacje o przesyłkach otrzymanych do doręczenia.
        * Informacje o rozliczeniach kontrahentów.

* ### Funkcje odbiorcy
    * #### Śledzenie statusu przesyłki
        Aplikacja umożliwia śledzenie statusu przesyłki od jej zarejestrowania w systemie do momentu dostarczenia przesyłki do odbiorcy.
    * #### Zarządzanie przesyłką
        Aplikacja umożliwia zmianę terminu dostarczenia przesyłki oraz przekierowanie jej pod inny adres.


## Logika Biznesowa

* Cykl życia przesyłki:
    1. Rejestracja przesyłki:

        Punkt startowy cyklu życia przesyłki inicjowany przez kontrahenta. Po wysłaniu żądania utworzenia przesyłki w systemie zostaje nadany unikalny numer przewozowy przesyłki.

    2. Wysłanie emaila/sms-a do odbiorcy z dostępem do panelu zarządzania przesyłką.

    3. Dodanie nowych statusów przesyłki przez pracowników firmy kurierskiej:
        * Po odebraniu przesyłki - wysłanie powiadomienia do odbiorcy.
        * Po dotarciu do punktów przeładunkowych.
        * Po wydaniu do doręczenia - wysłanie powiadomienia do odbiorcy.
    
    4. Doręczenie przesyłki:
        Wysłanie powiadomienia do odbiorcy oraz oznaczenie przesyłki jako zrealizowanej.

    Odbiorca może zmodyfikować termin oraz adres doręczenia, o ile przesyłka posiada status umożliwiający taką zmianę.

* Kontrahent:
    Dla każdego z kontrahentów definiowane są osobne stawki na określone rodzaje przesyłek (waga, rozmiar). Kontrahent może wywołać raport dotyczący rozliczenia, wtedy też zostaną wyliczone żądane dane.

## Lista Kluczowych jednostek biznesowych:
* ### Przesyłka:
    * Właściwości:
        * Adres Odbiorcy
        * Imię i Nazwisko/Nazwa Odbiorcy
        * Email Odbiorcy
        * Telefon Odbiorcy
        * Magazyn nadawczy
        * Data Odbioru
    * Funkcjonalności:
        * Zmiana statusu
        * Zmiana miejsca odbioru
        * Zmiana daty odbioru

* ### Kontrahent:
    * Właściwości:
        * Nazwa
        * Cennik
        * Lista magazynów nadawczych
    * Funkcjonalności:
        * Zarządzanie magazynami nadawczymi

## Serwisy

* ### Raportowanie
    * #### Rozliczenia
        Serwis służący do generowania raportów dla kontrahentów z informacjami na temat rozliczeń.
    * #### Przesyłki
        Serwis służący do generowania raportów dla kurierów z informacjami na temat przesyłek.