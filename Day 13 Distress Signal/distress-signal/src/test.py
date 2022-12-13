#https://adventofcode.com/2022/day/13
#from __future__ import annotations

from typing import Iterator
import itertools_recipes as ir
from functools import total_ordering
from collections import UserList
import ast
import enum


test_input="""
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
"""

          

def process_data(data:str) -> Iterator[tuple[list[int]]]:
    """transform the raw data into a procesable form"""
    for pack in ir.isplit(data.splitlines()):
        yield tuple( map(ast.literal_eval, pack ))
    
        
def get_raw_data(path:str="./input.txt") -> str:
    with open(path) as file:
        return file.read()



class Results(enum.Enum):
    CORRECT    = enum.auto()
    WRONG      = enum.auto()
    UNFINISHED = enum.auto()

            
def compare_int(a:int, b:int) -> Results:
    if a<b:
        return Results.CORRECT
    if a>b:
        return Results.WRONG
    return Results.UNFINISHED

def compare_list(left:list[...] ,right:list[...]) -> Results:
    for a,b in ir.zip_longest(left, right):
        if a is None:
            return Results.CORRECT
        if b is None:
            return Results.WRONG
        result = compare(a,b)
        if result is Results.UNFINISHED:
            continue
        else:
            return result
    return Results.UNFINISHED

def compare_mix(left:int|list[...], right:int|list[...]) -> Results:
    if isinstance(left, int) and isinstance(right, list):
        return compare_list([left], right)
    if isinstance(left, list) and isinstance(right, int):
        return compare_list(left, [right])
    raise ValueError("both have the same type")

def compare(left:list[...]|int, right:list[...]|int) -> Results:
    if isinstance(left, int) and isinstance(right, int):
        return compare_int(left, right)
    if isinstance(left, list) and isinstance(right, list):
        return compare_list(left, right)
    return compare_mix(left, right)
            

def part1(data:str, show=False) -> int:
    """part 1 of the puzzle """
    return sum( i for i, pair in enumerate(process_data(data),1) if compare(*pair) is Results.CORRECT)
    


@total_ordering
class Signal(UserList):

    def __eq__(self, otro):
        if not isinstance(otro, type(self)):
            return NotImplemented
        return self.data == otro.data

    def __lt__(self, otro):
        if not isinstance(otro, type(self)):
            return NotImplemented
        return compare(self.data, otro.data) is Results.CORRECT



def part2(data:str) -> str:
    """part 2 of the puzzle """
    packages = list(map(Signal,ir.chain.from_iterable(process_data(data))))
    a = Signal( [[2]] )
    b = Signal( [[6]] )
    packages.extend( [a,b])
    packages.sort()
    
    ia = packages.index(a)+1
    ib = packages.index(b)+1
    
    return ia*ib
     
    
 
    
   
def test1() -> bool:
    return part1(test_input, show=True) == 13

def test2() -> bool:
    return part2(test_input) == 140



data = get_raw_data()
assert test1(),"fail test 1"
print("solution part1:", part1(data)) # 5366
assert test2(),"fail test 2"
print("solution part2:", part2(data)) # 23391














