#![allow(dead_code)]

use crate::ast::*;
use crate::diagnostic::Span;
use crate::lexer::{SpannedToken, TokenKind};

pub struct Parser {
    tokens: Vec<SpannedToken>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<SpannedToken>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> &SpannedToken {
        &self.tokens[self.pos.min(self.tokens.len() - 1)]
    }

    fn peek_kind(&self) -> &TokenKind {
        &self.peek().kind
    }

    fn advance(&mut self) -> SpannedToken {
        let tok = self.peek().clone();
        if self.pos < self.tokens.len() - 1 {
            self.pos += 1;
        }
        tok
    }

    fn check(&self, kind: &TokenKind) -> bool {
        self.peek_kind() == kind
    }

    fn match_token(&mut self, kind: TokenKind) -> Result<SpannedToken, (String, Span)> {
        if self.check(&kind) {
            Ok(self.advance())
        } else {
            let tok = self.peek();
            Err((
                format!("Expected '{:?}', but found '{:?}'", kind, tok.kind),
                tok.span,
            ))
        }
    }

    fn skip_semicolons(&mut self) {
        while self.check(&TokenKind::Semicolon) {
            self.advance();
        }
    }

    pub fn parse_type(&mut self) -> Result<TypeNode, (String, Span)> {
        self.parse_union_type()
    }

    fn parse_union_type(&mut self) -> Result<TypeNode, (String, Span)> {
        let first = self.parse_primary_type()?;
        let mut members = vec![first];

        while self.check(&TokenKind::Pipe) {
            self.advance();
            members.push(self.parse_primary_type()?);
        }

        if members.len() == 1 {
            Ok(members.remove(0))
        } else {
            Ok(TypeNode::Union(members))
        }
    }

    fn parse_angle_type_args(&mut self) -> Result<Vec<TypeNode>, (String, Span)> {
        self.match_token(TokenKind::Less)?;
        let mut args = Vec::new();
        while !self.check(&TokenKind::Greater) && !self.check(&TokenKind::Eof) {
            args.push(self.parse_type()?);
            if self.check(&TokenKind::Comma) {
                self.advance();
                if self.check(&TokenKind::Greater) {
                    break;
                }
            } else {
                break;
            }
        }
        self.match_token(TokenKind::Greater)?;
        Ok(args)
    }

    fn parse_primary_type(&mut self) -> Result<TypeNode, (String, Span)> {
        let tok = self.advance();
        match tok.kind {
            TokenKind::Ident(ref name) => match name.as_str() {
                "Int" => Ok(TypeNode::Int),
                "Float" => Ok(TypeNode::Float),
                "Bool" => Ok(TypeNode::Bool),
                "String" => Ok(TypeNode::String),
                "Char" => Ok(TypeNode::Char),
                "Void" => Ok(TypeNode::Void),
                "Array" => {
                    let args = self.parse_angle_type_args()?;
                    if args.len() == 1 {
                        Ok(TypeNode::Array(Box::new(args[0].clone())))
                    } else {
                        Err((
                            "Array<T> expects exactly one type argument".into(),
                            tok.span,
                        ))
                    }
                }
                "Slice" => {
                    let args = self.parse_angle_type_args()?;
                    if args.len() == 1 {
                        Ok(TypeNode::Slice(Box::new(args[0].clone())))
                    } else {
                        Err((
                            "Slice<T> expects exactly one type argument".into(),
                            tok.span,
                        ))
                    }
                }
                "Set" => {
                    let args = self.parse_angle_type_args()?;
                    if args.len() == 1 {
                        Ok(TypeNode::Set(Box::new(args[0].clone())))
                    } else {
                        Err(("Set<T> expects exactly one type argument".into(), tok.span))
                    }
                }
                "Map" => {
                    let args = self.parse_angle_type_args()?;
                    if args.len() == 2 {
                        Ok(TypeNode::Map(
                            Box::new(args[0].clone()),
                            Box::new(args[1].clone()),
                        ))
                    } else {
                        Err((
                            "Map<K, V> expects exactly two type arguments".into(),
                            tok.span,
                        ))
                    }
                }
                "Option" => {
                    let args = self.parse_angle_type_args()?;
                    if args.len() == 1 {
                        Ok(TypeNode::Option(Box::new(args[0].clone())))
                    } else {
                        Err((
                            "Option<T> expects exactly one type argument".into(),
                            tok.span,
                        ))
                    }
                }
                "Task" => {
                    let args = self.parse_angle_type_args()?;
                    if args.len() == 1 {
                        Ok(TypeNode::Task(Box::new(args[0].clone())))
                    } else {
                        Err(("Task<T> expects exactly one type argument".into(), tok.span))
                    }
                }
                "Chan" => {
                    let args = self.parse_angle_type_args()?;
                    if args.len() == 1 {
                        Ok(TypeNode::Chan(Box::new(args[0].clone())))
                    } else {
                        Err(("Chan<T> expects exactly one type argument".into(), tok.span))
                    }
                }
                "Result" => {
                    let args = self.parse_angle_type_args()?;
                    if args.len() == 2 {
                        Ok(TypeNode::Result(
                            Box::new(args[0].clone()),
                            Box::new(args[1].clone()),
                        ))
                    } else {
                        Err((
                            "Result<T, E> expects exactly two type arguments".into(),
                            tok.span,
                        ))
                    }
                }
                custom => {
                    if self.check(&TokenKind::Less) {
                        let args = self.parse_angle_type_args()?;
                        Ok(TypeNode::Generic {
                            name: custom.to_string(),
                            args,
                        })
                    } else {
                        Ok(TypeNode::Custom(custom.to_string()))
                    }
                }
            },
            TokenKind::Fn => {
                self.match_token(TokenKind::LParen)?;
                let mut params = Vec::new();
                while !self.check(&TokenKind::RParen) && !self.check(&TokenKind::Eof) {
                    params.push(self.parse_type()?);
                    if self.check(&TokenKind::Comma) {
                        self.advance();
                    } else {
                        break;
                    }
                }
                self.match_token(TokenKind::RParen)?;
                self.match_token(TokenKind::Arrow)?;
                let ret = self.parse_type()?;
                Ok(TypeNode::Function {
                    params,
                    ret: Box::new(ret),
                })
            }
            TokenKind::Amp => {
                let is_mut = if self.check(&TokenKind::Mut) {
                    self.advance();
                    true
                } else {
                    false
                };
                let inner = self.parse_type()?;
                Ok(TypeNode::Ref(Box::new(inner), is_mut))
            }
            TokenKind::Star => {
                // Raw pointer type: *const T or *mut T
                let is_const = if self.check(&TokenKind::Const) || self.check(&TokenKind::Mut) {
                    if self.check(&TokenKind::Const) {
                        self.advance();
                        true
                    } else {
                        self.advance();
                        false
                    }
                } else {
                    return Err((
                        "Expected 'const' or 'mut' after '*' in pointer type".into(),
                        tok.span,
                    ));
                };
                let inner = self.parse_type()?;
                Ok(TypeNode::Ptr(Box::new(inner), is_const))
            }
            TokenKind::LBracket => {
                let inner = self.parse_type()?;
                self.match_token(TokenKind::RBracket)?;
                Ok(TypeNode::Array(Box::new(inner)))
            }
            TokenKind::LParen => {
                let mut items = Vec::new();
                if !self.check(&TokenKind::RParen) {
                    loop {
                        items.push(self.parse_type()?);
                        if self.check(&TokenKind::Comma) {
                            self.advance();
                            if self.check(&TokenKind::RParen) {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                }
                self.match_token(TokenKind::RParen)?;
                Ok(TypeNode::Tuple(items))
            }
            _ => Err((
                format!("Invalid type annotation '{:?}'", tok.kind),
                tok.span,
            )),
        }
    }

    fn parse_generic_params(&mut self) -> Result<Vec<GenericParam>, (String, Span)> {
        let mut params = Vec::new();
        if !self.check(&TokenKind::Less) {
            return Ok(params);
        }

        self.advance();
        while !self.check(&TokenKind::Greater) && !self.check(&TokenKind::Eof) {
            let name = match self.advance().kind {
                TokenKind::Ident(n) => n,
                other => {
                    return Err((
                        format!("Expected generic parameter name, got {:?}", other),
                        self.peek().span,
                    ));
                }
            };

            let mut bounds = Vec::new();
            if self.check(&TokenKind::Colon) {
                self.advance();
                loop {
                    bounds.push(self.parse_primary_type()?);
                    if self.check(&TokenKind::Plus) {
                        self.advance();
                    } else {
                        break;
                    }
                }
            }

            params.push(GenericParam { name, bounds });

            if self.check(&TokenKind::Comma) {
                self.advance();
                if self.check(&TokenKind::Greater) {
                    break;
                }
            } else {
                break;
            }
        }
        self.match_token(TokenKind::Greater)?;
        Ok(params)
    }

    pub fn parse_program(&mut self) -> Result<Program, (String, Span)> {
        let mut statements = Vec::new();
        self.skip_semicolons();

        while !self.check(&TokenKind::Eof) {
            statements.push(self.parse_statement()?);
            self.skip_semicolons();
        }

        Ok(Program { statements })
    }

    pub fn parse_statement(&mut self) -> Result<SpannedStmt, (String, Span)> {
        let mut attributes = Vec::new();
        while self.check(&TokenKind::At) {
            attributes.push(self.parse_attribute()?);
        }

        match self.peek_kind() {
            TokenKind::Async | TokenKind::Fn => self.parse_function(attributes),
            TokenKind::Struct => self.parse_struct_decl(attributes),
            TokenKind::Trait => self.parse_trait_decl(),
            TokenKind::Impl => self.parse_impl_block(),
            TokenKind::Operator => self.parse_operator_decl(attributes),
            TokenKind::Extern => self.parse_extern_block(attributes),
            TokenKind::Import => {
                let tok = self.advance();
                let mut path = match self.peek_kind() {
                    TokenKind::StrLit(s) => {
                        let s = s.clone();
                        self.advance();
                        s
                    }
                    TokenKind::Ident(n) => {
                        let n = n.clone();
                        self.advance();
                        n
                    }
                    _ => {
                        return Err((
                            "Expected module name or file path after 'import'".into(),
                            tok.span,
                        ));
                    }
                };
                while self.check(&TokenKind::Dot)
                    || self.check(&TokenKind::ColonColon)
                    || self.check(&TokenKind::Slash)
                {
                    self.advance();
                    match self.peek_kind() {
                        TokenKind::Ident(sub) => {
                            let sub_name = sub.clone();
                            path.push('.');
                            path.push_str(&sub_name);
                            self.advance();
                        }
                        _ => {
                            return Err((
                                "Expected module sub-identifier after '.'".into(),
                                tok.span,
                            ));
                        }
                    }
                }
                self.skip_semicolons();
                Ok(SpannedStmt {
                    kind: StmtKind::Import(path),
                    span: tok.span,
                })
            }
            TokenKind::Mod => {
                let tok = self.advance();
                let name = match self.peek_kind() {
                    TokenKind::Ident(n) => {
                        let n = n.clone();
                        self.advance();
                        n
                    }
                    _ => {
                        return Err((
                            "Expected module name identifier after 'mod'".into(),
                            tok.span,
                        ));
                    }
                };
                self.skip_semicolons();
                Ok(SpannedStmt {
                    kind: StmtKind::Mod(name),
                    span: tok.span,
                })
            }
            TokenKind::Enum => self.parse_enum_decl(),
            TokenKind::Type => self.parse_type_alias(),
            TokenKind::Const => self.parse_const(),
            TokenKind::Comptime => self.parse_comptime(),
            TokenKind::Let => self.parse_let(),
            TokenKind::Unsafe => self.parse_unsafe_block(),
            TokenKind::If => self.parse_if(),
            TokenKind::While => self.parse_while(),
            TokenKind::Loop => self.parse_loop(),
            TokenKind::Break => {
                let tok = self.advance();
                self.skip_semicolons();
                Ok(SpannedStmt {
                    kind: StmtKind::Break,
                    span: tok.span,
                })
            }
            TokenKind::Continue => {
                let tok = self.advance();
                self.skip_semicolons();
                Ok(SpannedStmt {
                    kind: StmtKind::Continue,
                    span: tok.span,
                })
            }
            TokenKind::Return => self.parse_return(),
            TokenKind::Print => self.parse_print(),
            TokenKind::Test => self.parse_test_block(),
            TokenKind::Assert => self.parse_assert(),
            TokenKind::Ident(_) => {
                if self.pos + 1 < self.tokens.len() {
                    let next_kind = &self.tokens[self.pos + 1].kind;
                    if matches!(
                        next_kind,
                        TokenKind::Equal
                            | TokenKind::PlusEqual
                            | TokenKind::MinusEqual
                            | TokenKind::StarEqual
                            | TokenKind::SlashEqual
                    ) {
                        return self.parse_assignment();
                    }
                }
                self.parse_expr_statement()
            }
            _ => self.parse_expr_statement(),
        }
    }

    fn parse_attribute(&mut self) -> Result<Attribute, (String, Span)> {
        let at_tok = self.match_token(TokenKind::At)?;
        let name = match self.peek_kind() {
            TokenKind::Ident(n) => {
                let n = n.clone();
                self.advance();
                n
            }
            _ => return Err(("Expected identifier after '@'".into(), self.peek().span)),
        };

        let mut args = Vec::new();
        let mut end_span = at_tok.span;
        if self.check(&TokenKind::LParen) {
            self.advance();
            while !self.check(&TokenKind::RParen) && !self.check(&TokenKind::Eof) {
                match self.peek_kind() {
                    TokenKind::Ident(arg) => {
                        args.push(arg.clone());
                        self.advance();
                    }
                    TokenKind::StrLit(arg) => {
                        args.push(arg.clone());
                        self.advance();
                    }
                    _ => {
                        return Err((
                            "Expected argument inside attribute ()".into(),
                            self.peek().span,
                        ));
                    }
                }
                if self.check(&TokenKind::Comma) {
                    self.advance();
                } else {
                    break;
                }
            }
            let rparen = self.match_token(TokenKind::RParen)?;
            end_span = rparen.span;
        }

        Ok(Attribute {
            name,
            args,
            span: at_tok.span.merge(end_span),
        })
    }

    fn parse_struct_decl(
        &mut self,
        attributes: Vec<Attribute>,
    ) -> Result<SpannedStmt, (String, Span)> {
        let st_tok = self.match_token(TokenKind::Struct)?;
        let name = match self.peek_kind() {
            TokenKind::Ident(n) => {
                let n = n.clone();
                self.advance();
                n
            }
            _ => return Err(("Expected struct name identifier".into(), self.peek().span)),
        };
        let type_params = self.parse_generic_params()?;

        self.match_token(TokenKind::LBrace)?;
        let mut fields = Vec::new();
        self.skip_semicolons();
        while !self.check(&TokenKind::RBrace) && !self.check(&TokenKind::Eof) {
            let f_name = match self.peek_kind() {
                TokenKind::Ident(n) => {
                    let n = n.clone();
                    self.advance();
                    n
                }
                _ => return Err(("Expected field name in struct".into(), self.peek().span)),
            };
            self.match_token(TokenKind::Colon)?;
            let f_ty = self.parse_type()?;
            fields.push((f_name, f_ty));
            if self.check(&TokenKind::Comma) {
                self.advance();
            }
            self.skip_semicolons();
        }
        let rbrace = self.match_token(TokenKind::RBrace)?;

        Ok(SpannedStmt {
            kind: StmtKind::StructDecl {
                attributes,
                name,
                type_params,
                fields,
            },
            span: st_tok.span.merge(rbrace.span),
        })
    }

    fn parse_trait_decl(&mut self) -> Result<SpannedStmt, (String, Span)> {
        let trait_tok = self.match_token(TokenKind::Trait)?;
        let name = match self.peek_kind() {
            TokenKind::Ident(n) => {
                let n = n.clone();
                self.advance();
                n
            }
            _ => return Err(("Expected trait name identifier".into(), self.peek().span)),
        };
        let type_params = self.parse_generic_params()?;

        self.match_token(TokenKind::LBrace)?;
        let mut associated_types = Vec::new();
        let mut methods = Vec::new();
        self.skip_semicolons();

        while !self.check(&TokenKind::RBrace) && !self.check(&TokenKind::Eof) {
            match self.peek_kind() {
                TokenKind::Type => associated_types.push(self.parse_associated_type_decl()?),
                TokenKind::Fn => methods.push(self.parse_trait_method_sig()?),
                other => {
                    return Err((
                        format!(
                            "Expected associated type or method in trait, got {:?}",
                            other
                        ),
                        self.peek().span,
                    ));
                }
            }
            self.skip_semicolons();
        }

        let rbrace = self.match_token(TokenKind::RBrace)?;
        Ok(SpannedStmt {
            kind: StmtKind::TraitDecl {
                name,
                type_params,
                associated_types,
                methods,
            },
            span: trait_tok.span.merge(rbrace.span),
        })
    }

    fn parse_associated_type_decl(&mut self) -> Result<AssociatedTypeDecl, (String, Span)> {
        let type_tok = self.match_token(TokenKind::Type)?;
        let name = match self.peek_kind() {
            TokenKind::Ident(n) => {
                let n = n.clone();
                self.advance();
                n
            }
            _ => return Err(("Expected associated type name".into(), self.peek().span)),
        };

        let mut bounds = Vec::new();
        if self.check(&TokenKind::Colon) {
            self.advance();
            loop {
                bounds.push(self.parse_primary_type()?);
                if self.check(&TokenKind::Plus) {
                    self.advance();
                } else {
                    break;
                }
            }
        }

        let mut default = None;
        if self.check(&TokenKind::Equal) {
            self.advance();
            default = Some(self.parse_type()?);
        }
        self.skip_semicolons();

        Ok(AssociatedTypeDecl {
            name,
            bounds,
            default,
            span: type_tok.span,
        })
    }

    fn parse_trait_method_sig(&mut self) -> Result<TraitMethodSig, (String, Span)> {
        let fn_tok = self.match_token(TokenKind::Fn)?;
        let name = match self.peek_kind() {
            TokenKind::Ident(n) => {
                let n = n.clone();
                self.advance();
                n
            }
            _ => return Err(("Expected trait method name".into(), self.peek().span)),
        };
        let type_params = self.parse_generic_params()?;

        self.match_token(TokenKind::LParen)?;
        let mut params = Vec::new();
        while !self.check(&TokenKind::RParen) && !self.check(&TokenKind::Eof) {
            params.push(self.parse_trait_method_param()?);
            if self.check(&TokenKind::Comma) {
                self.advance();
            } else {
                break;
            }
        }
        self.match_token(TokenKind::RParen)?;

        let mut ret_type = TypeNode::Void;
        if self.check(&TokenKind::Arrow) {
            self.advance();
            ret_type = self.parse_type()?;
        }
        self.skip_semicolons();

        Ok(TraitMethodSig {
            name,
            type_params,
            params,
            ret_type,
            span: fn_tok.span,
        })
    }

    fn parse_trait_method_param(&mut self) -> Result<TraitMethodParam, (String, Span)> {
        if self.check(&TokenKind::Amp) {
            self.advance();
            let is_mut = if self.check(&TokenKind::Mut) {
                self.advance();
                true
            } else {
                false
            };
            match self.advance().kind {
                TokenKind::Ident(name) if name == "self" => {
                    return Ok(TraitMethodParam {
                        name,
                        type_annot: Some(TypeNode::Ref(
                            Box::new(TypeNode::Custom("Self".to_string())),
                            is_mut,
                        )),
                        is_self: true,
                    });
                }
                other => {
                    return Err((
                        format!("Expected 'self' after '&' in trait method, got {:?}", other),
                        self.peek().span,
                    ));
                }
            }
        }

        let name = match self.advance().kind {
            TokenKind::Ident(n) => n,
            other => {
                return Err((
                    format!("Expected parameter name in trait method, got {:?}", other),
                    self.peek().span,
                ));
            }
        };

        if name == "self" && !self.check(&TokenKind::Colon) {
            return Ok(TraitMethodParam {
                name,
                type_annot: Some(TypeNode::Custom("Self".to_string())),
                is_self: true,
            });
        }

        self.match_token(TokenKind::Colon)?;
        let type_annot = self.parse_type()?;
        Ok(TraitMethodParam {
            name,
            type_annot: Some(type_annot),
            is_self: false,
        })
    }

    fn parse_impl_block(&mut self) -> Result<SpannedStmt, (String, Span)> {
        let impl_tok = self.match_token(TokenKind::Impl)?;
        let type_params = self.parse_generic_params()?;
        let first_type = self.parse_type()?;
        let (trait_ref, target_type) = if self.check(&TokenKind::For) {
            self.advance();
            let target = self.parse_type()?;
            (Some(first_type), target)
        } else {
            (None, first_type)
        };

        self.match_token(TokenKind::LBrace)?;
        let mut items = Vec::new();
        self.skip_semicolons();
        while !self.check(&TokenKind::RBrace) && !self.check(&TokenKind::Eof) {
            match self.peek_kind() {
                TokenKind::Type => {
                    let type_tok = self.match_token(TokenKind::Type)?;
                    let name = match self.peek_kind() {
                        TokenKind::Ident(n) => {
                            let n = n.clone();
                            self.advance();
                            n
                        }
                        _ => {
                            return Err((
                                "Expected associated type name in impl".into(),
                                self.peek().span,
                            ));
                        }
                    };
                    self.match_token(TokenKind::Equal)?;
                    let target = self.parse_type()?;
                    self.skip_semicolons();
                    items.push(ImplItem::AssociatedType {
                        name,
                        target,
                        span: type_tok.span,
                    });
                }
                TokenKind::Ident(s) if s == "pub" => {
                    self.advance();
                    let method = self.parse_function(Vec::new())?;
                    items.push(ImplItem::Method(Box::new(method)));
                }
                TokenKind::Async | TokenKind::Fn => {
                    let method = self.parse_function(Vec::new())?;
                    items.push(ImplItem::Method(Box::new(method)));
                }
                TokenKind::Operator => {
                    let operator = self.parse_operator_decl(Vec::new())?;
                    items.push(ImplItem::Method(Box::new(operator)));
                }
                other => {
                    return Err((
                        format!(
                            "Expected associated type, method, or operator in impl, got {:?}",
                            other
                        ),
                        self.peek().span,
                    ));
                }
            }
            self.skip_semicolons();
        }
        let rbrace = self.match_token(TokenKind::RBrace)?;

        Ok(SpannedStmt {
            kind: StmtKind::ImplBlock {
                type_params,
                trait_ref,
                target_type,
                items,
            },
            span: impl_tok.span.merge(rbrace.span),
        })
    }

    fn parse_operator_decl(
        &mut self,
        attributes: Vec<Attribute>,
    ) -> Result<SpannedStmt, (String, Span)> {
        let op_tok = self.match_token(TokenKind::Operator)?;
        let operator = self.parse_operator_symbol()?;
        let type_params = self.parse_generic_params()?;

        self.match_token(TokenKind::LParen)?;
        let mut params = Vec::new();
        while !self.check(&TokenKind::RParen) && !self.check(&TokenKind::Eof) {
            let p_name = match self.peek_kind() {
                TokenKind::Ident(n) => {
                    let n = n.clone();
                    self.advance();
                    n
                }
                _ => {
                    return Err((
                        "Expected parameter name in operator declaration".into(),
                        self.peek().span,
                    ));
                }
            };
            self.match_token(TokenKind::Colon)?;
            let p_type = self.parse_type()?;
            params.push((p_name, p_type));

            if self.check(&TokenKind::Comma) {
                self.advance();
            } else {
                break;
            }
        }
        self.match_token(TokenKind::RParen)?;

        let mut ret_type = TypeNode::Void;
        if self.check(&TokenKind::Arrow) {
            self.advance();
            ret_type = self.parse_type()?;
        }

        let body = self.parse_block()?;
        Ok(SpannedStmt {
            kind: StmtKind::Operator {
                attributes,
                operator,
                type_params,
                params,
                ret_type,
                body: body.0,
            },
            span: op_tok.span.merge(body.1),
        })
    }

    fn parse_operator_symbol(&mut self) -> Result<String, (String, Span)> {
        let tok = self.advance();
        let op = match tok.kind {
            TokenKind::Plus => "+".to_string(),
            TokenKind::Minus => "-".to_string(),
            TokenKind::Star => "*".to_string(),
            TokenKind::Slash => "/".to_string(),
            TokenKind::Percent => "%".to_string(),
            TokenKind::EqualEqual => "==".to_string(),
            TokenKind::BangEqual => "!=".to_string(),
            TokenKind::Less => "<".to_string(),
            TokenKind::LessEqual => "<=".to_string(),
            TokenKind::Greater => ">".to_string(),
            TokenKind::GreaterEqual => ">=".to_string(),
            TokenKind::Pipe => "|".to_string(),
            TokenKind::CustomOperator(op) => op,
            TokenKind::Ident(op) => op,
            other => {
                return Err((
                    format!("Expected operator symbol, got {:?}", other),
                    tok.span,
                ));
            }
        };
        Ok(op)
    }

    fn parse_extern_block(
        &mut self,
        attributes: Vec<Attribute>,
    ) -> Result<SpannedStmt, (String, Span)> {
        let ext_tok = self.match_token(TokenKind::Extern)?;

        // Optional ABI string: extern "C" { ... } (defaults to "C")
        let abi = match self.peek_kind() {
            TokenKind::StrLit(s) => {
                let s = s.clone();
                self.advance();
                s
            }
            _ => "C".to_string(),
        };

        self.match_token(TokenKind::LBrace)?;
        let mut functions = Vec::new();
        self.skip_semicolons();

        while !self.check(&TokenKind::RBrace) && !self.check(&TokenKind::Eof) {
            if !matches!(self.peek_kind(), TokenKind::Fn | TokenKind::Async) {
                return Err((
                    "Expected 'fn' declaration inside extern block".into(),
                    self.peek().span,
                ));
            }

            let fn_tok = self.match_token(TokenKind::Fn)?;
            let name = match self.peek_kind() {
                TokenKind::Ident(n) => {
                    let n = n.clone();
                    self.advance();
                    n
                }
                _ => return Err(("Expected external function name".into(), self.peek().span)),
            };

            self.match_token(TokenKind::LParen)?;
            let mut params = Vec::new();
            while !self.check(&TokenKind::RParen) && !self.check(&TokenKind::Eof) {
                let p_name = match self.peek_kind() {
                    TokenKind::Ident(n) => {
                        let n = n.clone();
                        self.advance();
                        n
                    }
                    _ => {
                        return Err((
                            "Expected parameter name in extern fn".into(),
                            self.peek().span,
                        ));
                    }
                };
                self.match_token(TokenKind::Colon)?;
                let p_type = self.parse_type()?;
                params.push((p_name, p_type));

                if self.check(&TokenKind::Comma) {
                    self.advance();
                } else {
                    break;
                }
            }
            self.match_token(TokenKind::RParen)?;

            let mut ret_type = TypeNode::Void;
            if self.check(&TokenKind::Arrow) {
                self.advance();
                ret_type = self.parse_type()?;
            }
            self.skip_semicolons();

            functions.push(ExternFnDecl {
                name,
                params,
                ret_type,
                span: fn_tok.span,
            });
        }
        let rbrace = self.match_token(TokenKind::RBrace)?;

        Ok(SpannedStmt {
            kind: StmtKind::ExternBlock {
                attributes,
                abi,
                functions,
            },
            span: ext_tok.span.merge(rbrace.span),
        })
    }

    fn parse_enum_decl(&mut self) -> Result<SpannedStmt, (String, Span)> {
        let enum_tok = self.match_token(TokenKind::Enum)?;
        let name = match self.peek_kind() {
            TokenKind::Ident(n) => {
                let n = n.clone();
                self.advance();
                n
            }
            _ => return Err(("Expected enum name identifier".into(), self.peek().span)),
        };
        let type_params = self.parse_generic_params()?;

        self.match_token(TokenKind::LBrace)?;
        let mut variants = Vec::new();
        self.skip_semicolons();
        while !self.check(&TokenKind::RBrace) && !self.check(&TokenKind::Eof) {
            let v_name = match self.peek_kind() {
                TokenKind::Ident(n) => {
                    let n = n.clone();
                    self.advance();
                    n
                }
                _ => return Err(("Expected variant name in enum".into(), self.peek().span)),
            };
            let mut payload = None;
            if self.check(&TokenKind::LParen) {
                self.advance();
                let mut payload_items = Vec::new();
                while !self.check(&TokenKind::RParen) && !self.check(&TokenKind::Eof) {
                    payload_items.push(self.parse_type()?);
                    if self.check(&TokenKind::Comma) {
                        self.advance();
                    } else {
                        break;
                    }
                }
                self.match_token(TokenKind::RParen)?;
                payload = match payload_items.len() {
                    0 => None,
                    1 => Some(payload_items.remove(0)),
                    _ => Some(TypeNode::Tuple(payload_items)),
                };
            }
            variants.push((v_name, payload));
            if self.check(&TokenKind::Comma) {
                self.advance();
            }
            self.skip_semicolons();
        }
        let rbrace = self.match_token(TokenKind::RBrace)?;

        Ok(SpannedStmt {
            kind: StmtKind::EnumDecl {
                name,
                type_params,
                variants,
            },
            span: enum_tok.span.merge(rbrace.span),
        })
    }

    fn parse_type_alias(&mut self) -> Result<SpannedStmt, (String, Span)> {
        let type_tok = self.match_token(TokenKind::Type)?;
        let name = match self.peek_kind() {
            TokenKind::Ident(n) => {
                let n = n.clone();
                self.advance();
                n
            }
            _ => return Err(("Expected type alias name".into(), self.peek().span)),
        };
        let type_params = self.parse_generic_params()?;
        self.match_token(TokenKind::Equal)?;
        let target = self.parse_type()?;
        self.skip_semicolons();

        Ok(SpannedStmt {
            kind: StmtKind::TypeAlias {
                name,
                type_params,
                target,
            },
            span: type_tok.span,
        })
    }

    fn parse_function(
        &mut self,
        attributes: Vec<Attribute>,
    ) -> Result<SpannedStmt, (String, Span)> {
        let (is_async, start_span) = if self.check(&TokenKind::Async) {
            let tok = self.advance();
            self.match_token(TokenKind::Fn)?;
            (true, tok.span)
        } else {
            let tok = self.match_token(TokenKind::Fn)?;
            (false, tok.span)
        };

        let name = match self.peek_kind() {
            TokenKind::Ident(n) => {
                let n = n.clone();
                self.advance();
                n
            }
            _ => return Err(("Expected function name identifier".into(), self.peek().span)),
        };
        let type_params = self.parse_generic_params()?;

        self.match_token(TokenKind::LParen)?;
        let mut params = Vec::new();
        while !self.check(&TokenKind::RParen) && !self.check(&TokenKind::Eof) {
            let (p_name, p_type) = if self.check(&TokenKind::Amp) {
                self.advance();
                let is_mut = if self.check(&TokenKind::Mut) {
                    self.advance();
                    true
                } else {
                    false
                };
                let name = match self.peek_kind() {
                    TokenKind::Ident(n) => {
                        let n = n.clone();
                        self.advance();
                        n
                    }
                    _ => {
                        return Err(("Expected parameter name after '&'".into(), self.peek().span));
                    }
                };
                let ty = if self.check(&TokenKind::Colon) {
                    self.advance();
                    self.parse_type()?
                } else {
                    TypeNode::Ref(Box::new(TypeNode::Custom("Self".to_string())), is_mut)
                };
                (name, ty)
            } else {
                let p_name = match self.peek_kind() {
                    TokenKind::Ident(n) => {
                        let n = n.clone();
                        self.advance();
                        n
                    }
                    _ => return Err(("Expected parameter name".into(), self.peek().span)),
                };

                if p_name == "self" && !self.check(&TokenKind::Colon) {
                    (p_name, TypeNode::Custom("Self".to_string()))
                } else {
                    self.match_token(TokenKind::Colon)?;
                    let p_type = self.parse_type()?;
                    (p_name, p_type)
                }
            };

            params.push((p_name, p_type));

            if self.check(&TokenKind::Comma) {
                self.advance();
            } else {
                break;
            }
        }
        self.match_token(TokenKind::RParen)?;

        let mut ret_type = TypeNode::Void;
        if self.check(&TokenKind::Arrow) {
            self.advance();
            ret_type = self.parse_type()?;
        }

        let body = self.parse_block()?;
        let span = start_span.merge(body.1);

        Ok(SpannedStmt {
            kind: StmtKind::Function {
                attributes,
                name,
                type_params,
                is_async,
                params,
                ret_type,
                body: body.0,
            },
            span,
        })
    }

    fn parse_block(&mut self) -> Result<(Vec<SpannedStmt>, Span), (String, Span)> {
        let lbrace = self.match_token(TokenKind::LBrace)?;
        let mut body = Vec::new();
        self.skip_semicolons();

        while !self.check(&TokenKind::RBrace) && !self.check(&TokenKind::Eof) {
            body.push(self.parse_statement()?);
            self.skip_semicolons();
        }

        let rbrace = self.match_token(TokenKind::RBrace)?;
        Ok((body, lbrace.span.merge(rbrace.span)))
    }

    fn parse_unsafe_block(&mut self) -> Result<SpannedStmt, (String, Span)> {
        let unsafe_tok = self.match_token(TokenKind::Unsafe)?;
        let (body, body_span) = self.parse_block()?;
        Ok(SpannedStmt {
            kind: StmtKind::UnsafeBlock(body),
            span: unsafe_tok.span.merge(body_span),
        })
    }

    fn parse_let(&mut self) -> Result<SpannedStmt, (String, Span)> {
        let let_tok = self.match_token(TokenKind::Let)?;
        let is_mut = if self.check(&TokenKind::Mut) {
            self.advance();
            true
        } else {
            false
        };

        if self.check(&TokenKind::LParen) || self.check(&TokenKind::LBrace) {
            if is_mut {
                return Err((
                    "Mutable destructuring bindings are not supported yet".into(),
                    let_tok.span,
                ));
            }
            let pattern = self.parse_destructure_pattern()?;
            let mut type_annot = None;
            if self.check(&TokenKind::Colon) {
                self.advance();
                type_annot = Some(self.parse_type()?);
            }
            self.match_token(TokenKind::Equal)?;
            let value = self.parse_expr()?;
            self.skip_semicolons();

            return Ok(SpannedStmt {
                kind: StmtKind::LetDestructure {
                    pattern,
                    type_annot,
                    value: value.clone(),
                },
                span: let_tok.span.merge(value.span),
            });
        }

        let name = match self.peek_kind() {
            TokenKind::Ident(n) => {
                let n = n.clone();
                self.advance();
                n
            }
            _ => {
                return Err((
                    "Expected variable identifier after 'let'".into(),
                    self.peek().span,
                ));
            }
        };

        let mut type_annot = None;
        if self.check(&TokenKind::Colon) {
            self.advance();
            type_annot = Some(self.parse_type()?);
        }

        self.match_token(TokenKind::Equal)?;
        let value = self.parse_expr()?;
        self.skip_semicolons();

        let span = let_tok.span.merge(value.span);
        Ok(SpannedStmt {
            kind: StmtKind::Let {
                name,
                is_mut,
                type_annot,
                value,
            },
            span,
        })
    }

    fn parse_destructure_pattern(&mut self) -> Result<Pattern, (String, Span)> {
        let tok = self.advance();
        match tok.kind {
            TokenKind::Ident(name) => {
                if name == "_" {
                    Ok(Pattern::Wildcard)
                } else if self.check(&TokenKind::LBrace) {
                    self.advance();
                    let mut fields = Vec::new();
                    while !self.check(&TokenKind::RBrace) && !self.check(&TokenKind::Eof) {
                        let field = match self.advance().kind {
                            TokenKind::Ident(n) => n,
                            other => {
                                return Err((
                                    format!(
                                        "Expected field name in struct pattern, got {:?}",
                                        other
                                    ),
                                    self.peek().span,
                                ));
                            }
                        };
                        let nested = if self.check(&TokenKind::Colon) {
                            self.advance();
                            Some(self.parse_destructure_pattern()?)
                        } else {
                            None
                        };
                        fields.push((field, nested));
                        if self.check(&TokenKind::Comma) {
                            self.advance();
                        }
                    }
                    self.match_token(TokenKind::RBrace)?;
                    Ok(Pattern::Struct { name, fields })
                } else {
                    Ok(Pattern::Identifier(name))
                }
            }
            TokenKind::LParen => {
                let mut patterns = Vec::new();
                while !self.check(&TokenKind::RParen) && !self.check(&TokenKind::Eof) {
                    patterns.push(self.parse_destructure_pattern()?);
                    if self.check(&TokenKind::Comma) {
                        self.advance();
                        if self.check(&TokenKind::RParen) {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                self.match_token(TokenKind::RParen)?;
                Ok(Pattern::Tuple(patterns))
            }
            TokenKind::LBrace => {
                let mut fields = Vec::new();
                while !self.check(&TokenKind::RBrace) && !self.check(&TokenKind::Eof) {
                    let field = match self.advance().kind {
                        TokenKind::Ident(n) => n,
                        other => {
                            return Err((
                                format!(
                                    "Expected field name in anonymous struct pattern, got {:?}",
                                    other
                                ),
                                self.peek().span,
                            ));
                        }
                    };
                    let nested = if self.check(&TokenKind::Colon) {
                        self.advance();
                        Some(self.parse_destructure_pattern()?)
                    } else {
                        None
                    };
                    fields.push((field, nested));
                    if self.check(&TokenKind::Comma) {
                        self.advance();
                    }
                }
                self.match_token(TokenKind::RBrace)?;
                Ok(Pattern::Struct {
                    name: String::new(),
                    fields,
                })
            }
            other => Err((
                format!("Invalid destructuring pattern {:?}", other),
                tok.span,
            )),
        }
    }

    fn parse_const(&mut self) -> Result<SpannedStmt, (String, Span)> {
        let const_tok = self.match_token(TokenKind::Const)?;
        let name = match self.peek_kind() {
            TokenKind::Ident(n) => {
                let n = n.clone();
                self.advance();
                n
            }
            _ => {
                return Err((
                    "Expected constant identifier after 'const'".into(),
                    self.peek().span,
                ));
            }
        };

        let mut type_annot = None;
        if self.check(&TokenKind::Colon) {
            self.advance();
            type_annot = Some(self.parse_type()?);
        }

        self.match_token(TokenKind::Equal)?;
        let value = self.parse_expr()?;
        self.skip_semicolons();

        let span = const_tok.span.merge(value.span);
        Ok(SpannedStmt {
            kind: StmtKind::Const {
                name,
                type_annot,
                value,
            },
            span,
        })
    }

    fn parse_comptime(&mut self) -> Result<SpannedStmt, (String, Span)> {
        let comp_tok = self.match_token(TokenKind::Comptime)?;
        let (body, body_span) = self.parse_block()?;
        Ok(SpannedStmt {
            kind: StmtKind::Comptime(body),
            span: comp_tok.span.merge(body_span),
        })
    }

    fn parse_assignment(&mut self) -> Result<SpannedStmt, (String, Span)> {
        let name_tok = self.advance();
        let name = match name_tok.kind {
            TokenKind::Ident(n) => n,
            _ => unreachable!(),
        };

        let op_tok = self.advance();
        let expr = self.parse_expr()?;
        self.skip_semicolons();

        let value = match op_tok.kind {
            TokenKind::Equal => expr,
            TokenKind::PlusEqual => SpannedExpr {
                span: name_tok.span.merge(expr.span),
                kind: ExprKind::Binary {
                    left: Box::new(SpannedExpr {
                        kind: ExprKind::Var(name.clone()),
                        span: name_tok.span,
                    }),
                    op: BinaryOpKind::Add,
                    right: Box::new(expr),
                },
            },
            TokenKind::MinusEqual => SpannedExpr {
                span: name_tok.span.merge(expr.span),
                kind: ExprKind::Binary {
                    left: Box::new(SpannedExpr {
                        kind: ExprKind::Var(name.clone()),
                        span: name_tok.span,
                    }),
                    op: BinaryOpKind::Sub,
                    right: Box::new(expr),
                },
            },
            TokenKind::StarEqual => SpannedExpr {
                span: name_tok.span.merge(expr.span),
                kind: ExprKind::Binary {
                    left: Box::new(SpannedExpr {
                        kind: ExprKind::Var(name.clone()),
                        span: name_tok.span,
                    }),
                    op: BinaryOpKind::Mul,
                    right: Box::new(expr),
                },
            },
            TokenKind::SlashEqual => SpannedExpr {
                span: name_tok.span.merge(expr.span),
                kind: ExprKind::Binary {
                    left: Box::new(SpannedExpr {
                        kind: ExprKind::Var(name.clone()),
                        span: name_tok.span,
                    }),
                    op: BinaryOpKind::Div,
                    right: Box::new(expr),
                },
            },
            _ => unreachable!(),
        };

        let span = name_tok.span.merge(value.span);
        Ok(SpannedStmt {
            kind: StmtKind::Assign {
                target: name,
                value,
            },
            span,
        })
    }

    fn parse_if(&mut self) -> Result<SpannedStmt, (String, Span)> {
        let if_tok = self.match_token(TokenKind::If)?;
        let condition = self.parse_expr()?;
        let (then_branch, then_span) = self.parse_block()?;

        let mut else_branch = None;
        let mut total_span = if_tok.span.merge(then_span);

        if self.check(&TokenKind::Else) {
            self.advance();
            if self.check(&TokenKind::If) {
                let nested_if = self.parse_if()?;
                total_span = total_span.merge(nested_if.span);
                else_branch = Some(vec![nested_if]);
            } else {
                let (e_body, e_span) = self.parse_block()?;
                total_span = total_span.merge(e_span);
                else_branch = Some(e_body);
            }
        }

        Ok(SpannedStmt {
            kind: StmtKind::If {
                condition,
                then_branch,
                else_branch,
            },
            span: total_span,
        })
    }

    fn parse_while(&mut self) -> Result<SpannedStmt, (String, Span)> {
        let while_tok = self.match_token(TokenKind::While)?;
        let condition = self.parse_expr()?;
        let (body, body_span) = self.parse_block()?;

        Ok(SpannedStmt {
            kind: StmtKind::While { condition, body },
            span: while_tok.span.merge(body_span),
        })
    }

    fn parse_loop(&mut self) -> Result<SpannedStmt, (String, Span)> {
        let loop_tok = self.match_token(TokenKind::Loop)?;
        let (body, body_span) = self.parse_block()?;

        Ok(SpannedStmt {
            kind: StmtKind::Loop { body },
            span: loop_tok.span.merge(body_span),
        })
    }

    fn parse_return(&mut self) -> Result<SpannedStmt, (String, Span)> {
        let ret_tok = self.match_token(TokenKind::Return)?;
        let mut ret_val = None;
        let mut span = ret_tok.span;

        if !self.check(&TokenKind::Semicolon)
            && !self.check(&TokenKind::RBrace)
            && !self.check(&TokenKind::Eof)
        {
            let expr = self.parse_expr()?;
            span = span.merge(expr.span);
            ret_val = Some(expr);
        }
        self.skip_semicolons();

        Ok(SpannedStmt {
            kind: StmtKind::Return(ret_val),
            span,
        })
    }

    fn parse_print(&mut self) -> Result<SpannedStmt, (String, Span)> {
        let print_tok = self.match_token(TokenKind::Print)?;
        self.match_token(TokenKind::LParen)?;

        let mut args = Vec::new();
        while !self.check(&TokenKind::RParen) && !self.check(&TokenKind::Eof) {
            args.push(self.parse_expr()?);
            if self.check(&TokenKind::Comma) {
                self.advance();
            } else {
                break;
            }
        }
        let rparen = self.match_token(TokenKind::RParen)?;
        self.skip_semicolons();

        Ok(SpannedStmt {
            kind: StmtKind::Print(args),
            span: print_tok.span.merge(rparen.span),
        })
    }

    fn parse_test_block(&mut self) -> Result<SpannedStmt, (String, Span)> {
        let test_tok = self.match_token(TokenKind::Test)?;
        let name = match self.peek_kind() {
            TokenKind::StrLit(s) => {
                let s = s.clone();
                self.advance();
                s
            }
            _ => {
                return Err((
                    "Expected test name string literal after 'test'".into(),
                    test_tok.span,
                ));
            }
        };
        let (body, body_span) = self.parse_block()?;
        Ok(SpannedStmt {
            kind: StmtKind::TestBlock { name, body },
            span: test_tok.span.merge(body_span),
        })
    }

    fn parse_assert(&mut self) -> Result<SpannedStmt, (String, Span)> {
        let assert_tok = self.match_token(TokenKind::Assert)?;
        self.match_token(TokenKind::LParen)?;
        let expr = self.parse_expr()?;
        let rparen = self.match_token(TokenKind::RParen)?;
        self.skip_semicolons();
        Ok(SpannedStmt {
            kind: StmtKind::Assert(expr),
            span: assert_tok.span.merge(rparen.span),
        })
    }

    fn parse_expr_statement(&mut self) -> Result<SpannedStmt, (String, Span)> {
        let expr = self.parse_expr()?;
        self.skip_semicolons();
        let span = expr.span;
        Ok(SpannedStmt {
            kind: StmtKind::Expr(expr),
            span,
        })
    }

    // --- Expressions ---
    pub fn parse_expr(&mut self) -> Result<SpannedExpr, (String, Span)> {
        if self.check(&TokenKind::Match) {
            self.parse_match()
        } else {
            self.parse_or()
        }
    }

    fn parse_match(&mut self) -> Result<SpannedExpr, (String, Span)> {
        let match_tok = self.match_token(TokenKind::Match)?;
        let value = self.parse_or()?;
        self.match_token(TokenKind::LBrace)?;

        let mut arms = Vec::new();
        while !self.check(&TokenKind::RBrace) && !self.check(&TokenKind::Eof) {
            let pat = self.parse_match_pattern()?;
            self.match_token(TokenKind::FatArrow)?;
            let body = self.parse_expr()?;
            if self.check(&TokenKind::Comma) {
                self.advance();
            }
            arms.push(MatchArm {
                pattern: pat,
                span: body.span,
                body: Box::new(body),
            });
        }
        let rbrace = self.match_token(TokenKind::RBrace)?;

        Ok(SpannedExpr {
            kind: ExprKind::Match {
                value: Box::new(value),
                arms,
            },
            span: match_tok.span.merge(rbrace.span),
        })
    }

    fn parse_match_pattern(&mut self) -> Result<MatchPattern, (String, Span)> {
        let tok = self.advance();
        match tok.kind {
            TokenKind::Some => {
                self.match_token(TokenKind::LParen)?;
                let var = match self.advance().kind {
                    TokenKind::Ident(n) => n,
                    other => {
                        return Err((
                            format!("Expected variable in Some pattern, got {:?}", other),
                            tok.span,
                        ));
                    }
                };
                self.match_token(TokenKind::RParen)?;
                Ok(MatchPattern::Some(var))
            }
            TokenKind::None => Ok(MatchPattern::None),
            TokenKind::Ok => {
                self.match_token(TokenKind::LParen)?;
                let var = match self.advance().kind {
                    TokenKind::Ident(n) => n,
                    other => {
                        return Err((
                            format!("Expected variable in Ok pattern, got {:?}", other),
                            tok.span,
                        ));
                    }
                };
                self.match_token(TokenKind::RParen)?;
                Ok(MatchPattern::Ok(var))
            }
            TokenKind::Err => {
                self.match_token(TokenKind::LParen)?;
                let var = match self.advance().kind {
                    TokenKind::Ident(n) => n,
                    other => {
                        return Err((
                            format!("Expected variable in Err pattern, got {:?}", other),
                            tok.span,
                        ));
                    }
                };
                self.match_token(TokenKind::RParen)?;
                Ok(MatchPattern::Err(var))
            }
            TokenKind::True | TokenKind::False => {
                Ok(MatchPattern::Literal(Box::new(SpannedExpr {
                    kind: ExprKind::Bool(matches!(tok.kind, TokenKind::True)),
                    span: tok.span,
                })))
            }
            TokenKind::IntLit(value) => Ok(MatchPattern::Literal(Box::new(SpannedExpr {
                kind: ExprKind::Int(value),
                span: tok.span,
            }))),
            TokenKind::Ident(name) => {
                if name == "_" {
                    Ok(MatchPattern::Wildcard)
                } else if self.check(&TokenKind::LParen) {
                    self.advance();
                    let inner = match self.advance().kind {
                        TokenKind::Ident(n) => n,
                        _ => return Err(("Expected variable in enum pattern".into(), tok.span)),
                    };
                    self.match_token(TokenKind::RParen)?;
                    Ok(MatchPattern::EnumVariant(name, Some(inner)))
                } else if name.chars().next().is_some_and(char::is_uppercase) {
                    Ok(MatchPattern::EnumVariant(name, None))
                } else {
                    Ok(MatchPattern::Var(name))
                }
            }
            _ => Err((format!("Invalid pattern '{:?}'", tok.kind), tok.span)),
        }
    }

    fn parse_or(&mut self) -> Result<SpannedExpr, (String, Span)> {
        let mut left = self.parse_and()?;
        while self.check(&TokenKind::PipePipe) {
            self.advance();
            let right = self.parse_and()?;
            let span = left.span.merge(right.span);
            left = SpannedExpr {
                kind: ExprKind::Binary {
                    left: Box::new(left),
                    op: BinaryOpKind::Or,
                    right: Box::new(right),
                },
                span,
            };
        }
        Ok(left)
    }

    fn parse_and(&mut self) -> Result<SpannedExpr, (String, Span)> {
        let mut left = self.parse_equality()?;
        while self.check(&TokenKind::AmpAmp) {
            self.advance();
            let right = self.parse_equality()?;
            let span = left.span.merge(right.span);
            left = SpannedExpr {
                kind: ExprKind::Binary {
                    left: Box::new(left),
                    op: BinaryOpKind::And,
                    right: Box::new(right),
                },
                span,
            };
        }
        Ok(left)
    }

    fn parse_equality(&mut self) -> Result<SpannedExpr, (String, Span)> {
        let mut left = self.parse_comparison()?;
        while matches!(
            self.peek_kind(),
            TokenKind::EqualEqual | TokenKind::BangEqual
        ) {
            let op_tok = self.advance();
            let op = match op_tok.kind {
                TokenKind::EqualEqual => BinaryOpKind::Equal,
                TokenKind::BangEqual => BinaryOpKind::NotEqual,
                _ => unreachable!(),
            };
            let right = self.parse_comparison()?;
            let span = left.span.merge(right.span);
            left = SpannedExpr {
                kind: ExprKind::Binary {
                    left: Box::new(left),
                    op,
                    right: Box::new(right),
                },
                span,
            };
        }
        Ok(left)
    }

    fn parse_comparison(&mut self) -> Result<SpannedExpr, (String, Span)> {
        let mut left = self.parse_term()?;
        while matches!(
            self.peek_kind(),
            TokenKind::Less
                | TokenKind::LessEqual
                | TokenKind::Greater
                | TokenKind::GreaterEqual
                | TokenKind::CustomOperator(_)
                | TokenKind::Is
        ) {
            let op_tok = self.advance();
            let op = match op_tok.kind {
                TokenKind::Is => {
                    let type_node = self.parse_type()?;
                    let span = left.span.merge(self.peek().span);
                    left = SpannedExpr {
                        kind: ExprKind::IsA {
                            value: Box::new(left),
                            type_node,
                        },
                        span,
                    };
                    continue;
                }
                TokenKind::CustomOperator(operator) => {
                    let right = self.parse_term()?;
                    let span = left.span.merge(right.span);
                    left = SpannedExpr {
                        kind: ExprKind::CustomBinary {
                            left: Box::new(left),
                            operator,
                            right: Box::new(right),
                        },
                        span,
                    };
                    continue;
                }
                TokenKind::Less => BinaryOpKind::Less,
                TokenKind::LessEqual => BinaryOpKind::LessEqual,
                TokenKind::Greater => BinaryOpKind::Greater,
                TokenKind::GreaterEqual => BinaryOpKind::GreaterEqual,
                _ => unreachable!(),
            };
            let right = self.parse_term()?;
            let span = left.span.merge(right.span);
            left = SpannedExpr {
                kind: ExprKind::Binary {
                    left: Box::new(left),
                    op,
                    right: Box::new(right),
                },
                span,
            };
        }
        Ok(left)
    }

    fn parse_term(&mut self) -> Result<SpannedExpr, (String, Span)> {
        let mut left = self.parse_factor()?;
        while matches!(self.peek_kind(), TokenKind::Plus | TokenKind::Minus) {
            let op_tok = self.advance();
            let op = match op_tok.kind {
                TokenKind::Plus => BinaryOpKind::Add,
                TokenKind::Minus => BinaryOpKind::Sub,
                _ => unreachable!(),
            };
            let right = self.parse_factor()?;
            let span = left.span.merge(right.span);
            left = SpannedExpr {
                kind: ExprKind::Binary {
                    left: Box::new(left),
                    op,
                    right: Box::new(right),
                },
                span,
            };
        }
        Ok(left)
    }

    fn parse_factor(&mut self) -> Result<SpannedExpr, (String, Span)> {
        let mut left = self.parse_unary()?;
        while matches!(
            self.peek_kind(),
            TokenKind::Star | TokenKind::Slash | TokenKind::Percent
        ) {
            let op_tok = self.advance();
            let op = match op_tok.kind {
                TokenKind::Star => BinaryOpKind::Mul,
                TokenKind::Slash => BinaryOpKind::Div,
                TokenKind::Percent => BinaryOpKind::Mod,
                _ => unreachable!(),
            };
            let right = self.parse_unary()?;
            let span = left.span.merge(right.span);
            left = SpannedExpr {
                kind: ExprKind::Binary {
                    left: Box::new(left),
                    op,
                    right: Box::new(right),
                },
                span,
            };
        }
        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<SpannedExpr, (String, Span)> {
        if self.check(&TokenKind::Move) {
            // `move expr` — explicit ownership transfer (Owned mode)
            let move_tok = self.advance();
            let operand = self.parse_unary()?;
            let span = move_tok.span.merge(operand.span);
            Ok(SpannedExpr {
                kind: ExprKind::Move(Box::new(operand)),
                span,
            })
        } else if self.check(&TokenKind::Unsafe)
            && self.pos + 1 < self.tokens.len()
            && self.tokens[self.pos + 1].kind == TokenKind::LBrace
        {
            // `unsafe { ... }` as an expression
            let unsafe_tok = self.advance();
            let (body, body_span) = self.parse_block()?;
            let span = unsafe_tok.span.merge(body_span);
            Ok(SpannedExpr {
                kind: ExprKind::UnsafeBlock(body),
                span,
            })
        } else if self.check(&TokenKind::Amp) {
            let amp_tok = self.advance();
            let is_mut = if self.check(&TokenKind::Mut) {
                self.advance();
                true
            } else {
                false
            };
            let operand = self.parse_unary()?;
            let span = amp_tok.span.merge(operand.span);
            Ok(SpannedExpr {
                kind: ExprKind::Borrow {
                    expr: Box::new(operand),
                    is_mut,
                },
                span,
            })
        } else if matches!(self.peek_kind(), TokenKind::Minus | TokenKind::Bang) {
            let op_tok = self.advance();
            let op = match op_tok.kind {
                TokenKind::Minus => UnaryOpKind::Neg,
                TokenKind::Bang => UnaryOpKind::Not,
                _ => unreachable!(),
            };
            let operand = self.parse_unary()?;
            let span = op_tok.span.merge(operand.span);
            Ok(SpannedExpr {
                kind: ExprKind::Unary {
                    op,
                    expr: Box::new(operand),
                },
                span,
            })
        } else {
            self.parse_postfix()
        }
    }

    fn parse_postfix(&mut self) -> Result<SpannedExpr, (String, Span)> {
        let mut expr = self.parse_primary()?;

        loop {
            if self.check(&TokenKind::LParen) {
                self.advance();
                let mut args = Vec::new();
                while !self.check(&TokenKind::RParen) && !self.check(&TokenKind::Eof) {
                    args.push(self.parse_expr()?);
                    if self.check(&TokenKind::Comma) {
                        self.advance();
                    } else {
                        break;
                    }
                }
                let rparen = self.match_token(TokenKind::RParen)?;
                let span = expr.span.merge(rparen.span);

                let callee_name = match expr.kind {
                    ExprKind::Var(name) => name,
                    ExprKind::Dot { target, field } => {
                        args.insert(0, *target);
                        format!("__aether_method::{}", field)
                    }
                    _ => {
                        return Err((
                            "Only named identifiers or methods can be called directly".into(),
                            expr.span,
                        ));
                    }
                };

                expr = SpannedExpr {
                    kind: ExprKind::Call {
                        callee: callee_name,
                        args,
                    },
                    span,
                };
            } else if self.check(&TokenKind::LBracket) {
                self.advance();
                let idx = self.parse_expr()?;
                let rbracket = self.match_token(TokenKind::RBracket)?;
                let span = expr.span.merge(rbracket.span);
                expr = SpannedExpr {
                    kind: ExprKind::Index {
                        target: Box::new(expr),
                        index: Box::new(idx),
                    },
                    span,
                };
            } else if self.check(&TokenKind::Dot) {
                self.advance();
                let field = match self.advance().kind {
                    TokenKind::Ident(f) => f,
                    _ => return Err(("Expected field name after '.'".into(), expr.span)),
                };
                expr = SpannedExpr {
                    kind: ExprKind::Dot {
                        target: Box::new(expr),
                        field,
                    },
                    span: self.peek().span,
                };
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<SpannedExpr, (String, Span)> {
        let tok = self.peek().clone();
        match tok.kind {
            TokenKind::IntLit(n) => {
                self.advance();
                Ok(SpannedExpr {
                    kind: ExprKind::Int(n),
                    span: tok.span,
                })
            }
            TokenKind::FloatLit(f) => {
                self.advance();
                Ok(SpannedExpr {
                    kind: ExprKind::Float(f),
                    span: tok.span,
                })
            }
            TokenKind::StrLit(s) => {
                self.advance();
                Ok(SpannedExpr {
                    kind: ExprKind::Str(s),
                    span: tok.span,
                })
            }
            TokenKind::CharLit(c) => {
                self.advance();
                Ok(SpannedExpr {
                    kind: ExprKind::Char(c),
                    span: tok.span,
                })
            }
            TokenKind::True => {
                self.advance();
                Ok(SpannedExpr {
                    kind: ExprKind::Bool(true),
                    span: tok.span,
                })
            }
            TokenKind::False => {
                self.advance();
                Ok(SpannedExpr {
                    kind: ExprKind::Bool(false),
                    span: tok.span,
                })
            }
            TokenKind::Null => {
                self.advance();
                Ok(SpannedExpr {
                    kind: ExprKind::Null,
                    span: tok.span,
                })
            }
            TokenKind::Some => {
                self.advance();
                self.match_token(TokenKind::LParen)?;
                let inner = self.parse_expr()?;
                let rparen = self.match_token(TokenKind::RParen)?;
                Ok(SpannedExpr {
                    kind: ExprKind::Some(Box::new(inner)),
                    span: tok.span.merge(rparen.span),
                })
            }
            TokenKind::None => {
                self.advance();
                Ok(SpannedExpr {
                    kind: ExprKind::None,
                    span: tok.span,
                })
            }
            TokenKind::Ok => {
                self.advance();
                self.match_token(TokenKind::LParen)?;
                let inner = self.parse_expr()?;
                let rparen = self.match_token(TokenKind::RParen)?;
                Ok(SpannedExpr {
                    kind: ExprKind::Ok(Box::new(inner)),
                    span: tok.span.merge(rparen.span),
                })
            }
            TokenKind::Err => {
                self.advance();
                self.match_token(TokenKind::LParen)?;
                let inner = self.parse_expr()?;
                let rparen = self.match_token(TokenKind::RParen)?;
                Ok(SpannedExpr {
                    kind: ExprKind::Err(Box::new(inner)),
                    span: tok.span.merge(rparen.span),
                })
            }
            TokenKind::Task => {
                self.advance();
                let inner = self.parse_expr()?;
                let span = tok.span.merge(inner.span);
                Ok(SpannedExpr {
                    kind: ExprKind::Task(Box::new(inner)),
                    span,
                })
            }
            TokenKind::Await => {
                self.advance();
                let inner = self.parse_expr()?;
                let span = tok.span.merge(inner.span);
                Ok(SpannedExpr {
                    kind: ExprKind::Await(Box::new(inner)),
                    span,
                })
            }
            TokenKind::Spawn => {
                self.advance();
                let callee = match self.advance().kind {
                    TokenKind::Ident(n) => n,
                    _ => return Err(("Expected function name after 'spawn'".into(), tok.span)),
                };
                self.match_token(TokenKind::LParen)?;
                let mut args = Vec::new();
                while !self.check(&TokenKind::RParen) && !self.check(&TokenKind::Eof) {
                    args.push(self.parse_expr()?);
                    if self.check(&TokenKind::Comma) {
                        self.advance();
                    } else {
                        break;
                    }
                }
                let rparen = self.match_token(TokenKind::RParen)?;
                Ok(SpannedExpr {
                    kind: ExprKind::Spawn { callee, args },
                    span: tok.span.merge(rparen.span),
                })
            }
            TokenKind::Chan => {
                self.advance();
                let elem_ty = if self.check(&TokenKind::Less) {
                    self.advance();
                    let t = self.parse_type()?;
                    self.match_token(TokenKind::Greater)?;
                    t
                } else {
                    TypeNode::Infer
                };
                self.match_token(TokenKind::LParen)?;
                let rparen = self.match_token(TokenKind::RParen)?;
                Ok(SpannedExpr {
                    kind: ExprKind::MakeChan(Box::new(elem_ty)),
                    span: tok.span.merge(rparen.span),
                })
            }
            TokenKind::Ident(name) => {
                self.advance();
                if self.check(&TokenKind::ColonColon) {
                    self.advance();
                    let variant_name = match self.advance().kind {
                        TokenKind::Ident(name) => name,
                        other => {
                            return Err((
                                format!("Expected enum variant name, got {:?}", other),
                                tok.span,
                            ));
                        }
                    };
                    let payload = if self.check(&TokenKind::LParen) {
                        self.advance();
                        let mut values = Vec::new();
                        while !self.check(&TokenKind::RParen) && !self.check(&TokenKind::Eof) {
                            values.push(self.parse_expr()?);
                            if self.check(&TokenKind::Comma) {
                                self.advance();
                            } else {
                                break;
                            }
                        }
                        self.match_token(TokenKind::RParen)?;
                        match values.len() {
                            0 => None,
                            1 => Some(Box::new(values.remove(0))),
                            _ => Some(Box::new(SpannedExpr {
                                kind: ExprKind::Tuple(values),
                                span: tok.span,
                            })),
                        }
                    } else {
                        None
                    };
                    return Ok(SpannedExpr {
                        kind: ExprKind::EnumVariantConstruct {
                            enum_name: name,
                            variant_name,
                            payload,
                        },
                        span: tok.span,
                    });
                }
                if name == "set" && self.check(&TokenKind::LBrace) {
                    self.advance();
                    let mut elements = Vec::new();
                    while !self.check(&TokenKind::RBrace) && !self.check(&TokenKind::Eof) {
                        elements.push(self.parse_expr()?);
                        if self.check(&TokenKind::Comma) {
                            self.advance();
                            if self.check(&TokenKind::RBrace) {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                    let rbrace = self.match_token(TokenKind::RBrace)?;
                    return Ok(SpannedExpr {
                        kind: ExprKind::Set(elements),
                        span: tok.span.merge(rbrace.span),
                    });
                }
                Ok(SpannedExpr {
                    kind: ExprKind::Var(name),
                    span: tok.span,
                })
            }
            TokenKind::LParen => {
                self.advance();
                if self.check(&TokenKind::RParen) {
                    let rparen = self.match_token(TokenKind::RParen)?;
                    return Ok(SpannedExpr {
                        kind: ExprKind::Tuple(Vec::new()),
                        span: tok.span.merge(rparen.span),
                    });
                }
                let expr = self.parse_expr()?;
                if self.check(&TokenKind::Comma) {
                    let mut elements = vec![expr];
                    while self.check(&TokenKind::Comma) {
                        self.advance();
                        if self.check(&TokenKind::RParen) {
                            break;
                        }
                        elements.push(self.parse_expr()?);
                    }
                    let rparen = self.match_token(TokenKind::RParen)?;
                    return Ok(SpannedExpr {
                        kind: ExprKind::Tuple(elements),
                        span: tok.span.merge(rparen.span),
                    });
                }
                let rparen = self.match_token(TokenKind::RParen)?;
                Ok(SpannedExpr {
                    kind: expr.kind,
                    span: tok.span.merge(rparen.span),
                })
            }
            TokenKind::LBracket => {
                self.advance();
                let mut elements = Vec::new();
                while !self.check(&TokenKind::RBracket) && !self.check(&TokenKind::Eof) {
                    elements.push(self.parse_expr()?);
                    if self.check(&TokenKind::Comma) {
                        self.advance();
                    } else {
                        break;
                    }
                }
                let rbracket = self.match_token(TokenKind::RBracket)?;
                Ok(SpannedExpr {
                    kind: ExprKind::Array(elements),
                    span: tok.span.merge(rbracket.span),
                })
            }
            TokenKind::LBrace => {
                // Map or block
                self.advance();
                // Check if Map literal { key: val }
                if !self.check(&TokenKind::RBrace) && !self.check(&TokenKind::Eof) {
                    let first_expr = self.parse_expr()?;
                    if self.check(&TokenKind::Colon) {
                        self.advance();
                        let first_val = self.parse_expr()?;
                        let mut entries = vec![(first_expr, first_val)];
                        while self.check(&TokenKind::Comma) {
                            self.advance();
                            if self.check(&TokenKind::RBrace) {
                                break;
                            }
                            let k = self.parse_expr()?;
                            self.match_token(TokenKind::Colon)?;
                            let v = self.parse_expr()?;
                            entries.push((k, v));
                        }
                        let rbrace = self.match_token(TokenKind::RBrace)?;
                        return Ok(SpannedExpr {
                            kind: ExprKind::Map(entries),
                            span: tok.span.merge(rbrace.span),
                        });
                    } else {
                        // It's a statement block
                        let mut stmts = vec![SpannedStmt {
                            span: first_expr.span,
                            kind: StmtKind::Expr(first_expr),
                        }];
                        self.skip_semicolons();
                        while !self.check(&TokenKind::RBrace) && !self.check(&TokenKind::Eof) {
                            stmts.push(self.parse_statement()?);
                            self.skip_semicolons();
                        }
                        let rbrace = self.match_token(TokenKind::RBrace)?;
                        return Ok(SpannedExpr {
                            kind: ExprKind::Block(stmts),
                            span: tok.span.merge(rbrace.span),
                        });
                    }
                }
                let rbrace = self.match_token(TokenKind::RBrace)?;
                Ok(SpannedExpr {
                    kind: ExprKind::Map(Vec::new()),
                    span: tok.span.merge(rbrace.span),
                })
            }
            _ => Err((
                format!("Unexpected token in expression: '{:?}'", tok.kind),
                tok.span,
            )),
        }
    }
}

pub fn parse(tokens: Vec<SpannedToken>) -> Result<Program, (String, Span)> {
    Parser::new(tokens).parse_program()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::tokenize;

    #[test]
    fn test_parse_modern_types() {
        let src = "fn find_user(id: Int) -> Option<String> { if id > 0 { return Some(\"User\"); } return None; }";
        let tokens = tokenize(src).unwrap();
        let program = parse(tokens).unwrap();
        assert_eq!(program.statements.len(), 1);
    }
}
