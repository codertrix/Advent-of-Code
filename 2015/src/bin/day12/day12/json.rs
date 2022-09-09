// ---------------------------------------------------
//  Advent of Code 2015
//  Day 12: JSAbacusFramework.io
//
//  Copyright © 2022 Michael Fenske. See LICENSE.txt.
// ---------------------------------------------------

use std::collections::HashMap;

#[derive(PartialEq)]
pub enum Json<'a> {
    Array(Vec<Json<'a>>),
    Dict(HashMap<&'a str, Json<'a>>),
    Number(isize),
    Str(&'a str)
}

#[derive(Copy, Clone, PartialEq)]
enum State {
    Start,
    Array,
    Dict,
    Key,
    Value,
    Number,
    Str
}

// -----------------------------------------------------
//  Important hint:
//  This is not a full implementation of a JSON parser!
//  Just enough to solve these puzzles.
// -----------------------------------------------------
pub struct JsonParser<'a> {
    input: &'a str,
    state: State,
    states: Vec<State>,
    array_stack: Vec<Vec<Json<'a>>>,
    dict_stack: Vec<HashMap<&'a str, Json<'a>>>,
    keys: Vec<&'a str>
}

impl<'a> JsonParser<'a> {
    pub fn from_str(input: &'a str) -> Self {
        JsonParser {
            input,
            state: State::Start,
            states: Vec::new(),
            array_stack: Vec::new(),
            dict_stack: Vec::new(),
            keys: Vec::new(),
        }
    }

    pub fn parse(&mut self) -> Option<Json> {
        let mut start_idx = 0;

        self.state = State::Start;

        for (idx, c) in self.input.chars().enumerate() {
            if c.is_whitespace() && self.state != State::Number { continue; }

            match self.state {
                State::Start => {
                    self.states.push(self.state);

                    match c {
                        '[' => {
                            self.array_stack.push(Vec::new());
                            self.push_state(State::Array);
                        }
                        '{' => {
                            self.dict_stack.push(HashMap::new());
                            self.push_state(State::Dict);
                        }
                        _ => break
                    }
                }
                State::Array => {
                    match c {
                        ']' => {
                            self.pop_state();

                            if let Some(array) = self.array_stack.pop() {
                                let json = Json::Array(array);

                                if self.state == State::Start {
                                    return Some(json);
                                }

                                self.add(json);
                            }
                        }
                        '[' => {
                            self.array_stack.push(Vec::new());
                            self.push_state(State::Array);
                        }
                        '{' => {
                            self.dict_stack.push(HashMap::new());
                            self.push_state(State::Dict);
                        }
                        '-' | '0'..='9' => {
                            start_idx = idx;
                            self.push_state(State::Number);
                        }
                        '"' => {
                            start_idx = idx + 1;
                            self.push_state(State::Str);
                        }
                        ',' => (),
                        _ => break
                    }
                }
                State::Dict => {
                    match c {
                        '}' => {
                            self.pop_state();

                            if let Some(dict) = self.dict_stack.pop() {
                                let json = Json::Dict(dict);

                                if self.state == State::Start {
                                    return Some(json);
                                }

                                self.add(json);
                            }
                        }
                        '"' => {
                            start_idx = idx + 1;
                            self.push_state(State::Key);
                        }
                        ',' => (),
                        _ => break
                    }
                }
                State::Key => {
                    if c == '"' {
                        self.state = State::Value;
                        self.keys.push(&self.input[start_idx..idx]);
                    }
                }
                State::Value => {
                    match c {
                        '[' => {
                            self.array_stack.push(Vec::new());
                            self.state = State::Array;
                        }
                        '{' => {
                            self.dict_stack.push(HashMap::new());
                            self.state = State::Dict;
                        }
                        '-' | '0'..='9' => {
                            if self.keys.is_empty() { break; }

                            start_idx = idx;
                            self.state = State::Number;
                        }
                        '"' => {
                            start_idx = idx + 1;
                            self.state = State::Str;
                        }
                        ':' => (),
                        _ => break
                    }
                }
                State::Number => {
                    if !c.is_ascii_digit() {
                        self.pop_state();

                        if let Ok(number) = self.input[start_idx..idx].parse::<isize>() {
                            self.add(Json::Number(number));
                        }

                        if c == ']' {
                            self.pop_state();

                            if let Some(array) = self.array_stack.pop() {
                                let json = Json::Array(array);

                                if self.state == State::Start {
                                    return Some(json);
                                }

                                self.add(json);
                            }
                        }

                        if c == '}' {
                            self.pop_state();

                            if let Some(dict) = self.dict_stack.pop() {
                                let json = Json::Dict(dict);

                                if self.state == State::Start {
                                    return Some(json);
                                }

                                self.add(json);
                            }
                        }
                    }
                }
                State::Str => {
                    if c == '"' {
                        self.pop_state();
                        self.add(Json::Str(&self.input[start_idx..idx]));
                    }
                }
            }
        }

        None
    }

    fn  add(&mut self, json: Json<'a>) {
        match self.state {
            State::Array => {
                let idx = self.array_stack.len() - 1;
                self.array_stack[idx].push(json);
            }
            State::Dict => {
                let idx = self.dict_stack.len() - 1;

                if let Some(key) = self.keys.pop() {
                    self.dict_stack[idx].insert(key, json);
                }
            }
            _ => ()
        }
    }

    fn pop_state(&mut self) {
        if let Some(state) = self.states.pop() {
            self.state = state;
        }
    }

    fn push_state(&mut self, state: State) {
        self.states.push(self.state);
        self.state = state;
    }
}
