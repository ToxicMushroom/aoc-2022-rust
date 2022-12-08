% prolog
:- use_module(library(clpfd)).
:- use_module(library(dcg/basics)).

elve1([N|Ns]) --> integer(N), eol, elve(Ns).
elve([N|Ns]) --> integer(N), eol, elve(Ns).
elve([]) --> [].

% Obsolete due to dcg/basics' integer//1 predicate.
% integer2(N) --> numbers(L), { number_codes(N, L) }.
% numbers([A|As]) --> [A], { char_type(A, digit) }, all_digits(As).
% all_digits([A|As]) --> [A], { char_type(A, digit) }, all_digits(As).
% all_digits([]) --> [].

elves([E|Es]) --> elve1(E), eol, elves(Es).
elves([]) --> [].

insert(E, [], [E]).
insert(E, [H|L], N) :- 
    (E #=< H, N = [E| [H|L]]); (E #> H, N = [H | Rest], insert(E, L, Rest)).

find_3max([], R, R).
find_3max([H|T], R, RR) :- 
    sum_list(H, S),
    ((length(R, 3), insert(S, R, N), N = [_|R2], find_3max(T, R2, RR));
    (length(R, LR), LR #< 3, insert(S, R, R2), find_3max(T, R2, RR))).

solve(R1, R2) :-
    phrase_from_file(elves(N), "../input/day1"),
    find_3max(N, [], M3),
    [_, _, R1] = M3,
    sum_list(M3, R2),
    writeln(R1),
    writeln(R2).
