% prolog
:- use_module(library(clpfd)).
:- use_module(library(dcg/basics)).
% :- table numbers//0.

elve1([N|Ns]) --> integer2(N), eol, elve(Ns).
elve([N|Ns]) --> integer2(N), eol, elve(Ns).
elve([]) --> [].

integer2(N) --> numbers(L), { number_codes(N, L) }.
numbers([A|As]) --> [A], { char_type(A, digit) }, all_digits(As).
all_digits([A|As]) --> [A], { char_type(A, digit) }, all_digits(As).
all_digits([]) --> [].

elves([E|Es]) --> elve1(E), eol, elves(Es).
elves([]) --> [].

sum([], 0).
sum([H|T], A) :- 
    A #= H + A2, 
    sum(T, A2).

find_max([B], B).    
find_max([H|T], A) :-
    A #= max(H, A2),
    find_max(T, A2).

find_max_sum([B], R) :- sum(B, R).
find_max_sum([H|T], MS) :-
    MS #= max(RS, RMS),
    sum(H, RS),
    find_max_sum(T, RMS).

insert(E, [], [E]).
insert(E, [H|L], N) :- 
    (E #=< H, N=E|[H|L]);
    (E #> H, N = [H | Rest], insert(E, L, N)).

% find_max_3sum([B], R) :- .
% find_max_3sum([H|T], MS) :- 
find_3max([H|T], R) :- 
    (length(R, 3), insert(sum(H), R, N), N = [HA|R2], find_3max(T, R2));
    (length(R, LR), LR #< 3, insert(sum(H), R, R2), find_3max(T, R2)).
    
    

solve(R, R2) :-
    phrase_from_file(elves(N), "../input/day1"),
    find_max_sum(N, R),
    find_max_3sum(N, R).
