use super::AsStr;

pub trait Enum: std::fmt::Debug + Clone {
    fn get_name(self: &Self) -> &str;
    fn has_value(self: &Self, value: &str) -> bool;
}

pub enum InputTypeSpecRef<'s> {
    InputType(&'s str),
    Scalar(&'s str),
    Enum(&'s str),
}

pub trait InputTypeSpec {
    fn get_ref(self: &Self) -> InputTypeSpecRef<'_>;
}

pub enum LiteralRef<'r> {
    Null,
    Int(&'r i64),
    Float(&'r f64),
    String(&'r str),
    Boolean(&'r bool),
    EnumValue(&'r str),
}

pub trait Literal {
    fn get_ref(self: &Self) -> LiteralRef<'_>;
}

pub enum ArrayLiteralRef<'r, S: AsStr<'r>> {
    Int(&'r [i64]),
    Float(&'r [f64]),
    String(&'r [S]),
    Boolean(&'r [bool]),
}

pub trait ArrayLiteral {
    fn get_ref(self: &Self) -> ArrayLiteralRef<'_, impl AsStr<'_>>;
}

pub trait LiteralFieldSpec<
    T,
    L: Literal,
    AL: ArrayLiteral,
    AV: ArgumentValue,
    S: ServerDirectiveInvocation<T, L, AL, AV, Self, AFS, N, FD, FSA>,
    AFS: ArrayFieldSpec<T, L, AL, AV, S, Self, FD, FSA, N>,
    FD: FieldDefinition<N>,
    FSA: FieldSelectionArgument<T, L, AL, AV, S, Self, AFS, N, FD>,
    N: NonCallableFieldSpec<T, L, AL, AV, S, Self, AFS, FD, FSA>,
>: Sized
{
    fn get_type(self: &Self) -> &T;
    fn get_default_value(self: &Self) -> &Option<Option<L>>;
    fn get_directive_invocations(self: &Self) -> &[(&str, &S)];
    fn has_directive_invocation(self: &Self, name: &str) -> bool;
}

pub trait ArrayFieldSpec<
    T,
    L: Literal,
    AL: ArrayLiteral,
    AV: ArgumentValue,
    S: ServerDirectiveInvocation<T, L, AL, AV, LFS, Self, N, FD, FSA>,
    LFS: LiteralFieldSpec<T, L, AL, AV, S, Self, FD, FSA, N>,
    FD: FieldDefinition<N>,
    FSA: FieldSelectionArgument<T, L, AL, AV, S, LFS, Self, N, FD>,
    N: NonCallableFieldSpec<T, L, AL, AV, S, LFS, Self, FD, FSA>,
>: Sized
{
    fn get_type(self: &Self) -> &N;
    fn get_nullable(self: &Self) -> bool;
    fn get_default_value(self: &Self) -> &Option<Option<AL>>;
    fn directive_invocations(self: &Self) -> &[S];
}

pub enum NonCallableFieldSpecRef<'r, LFS, AFS> {
    Literal(&'r LFS),
    Array(&'r AFS),
}

pub trait NonCallableFieldSpec<
    T,
    L: Literal,
    AL: ArrayLiteral,
    AV: ArgumentValue,
    S: ServerDirectiveInvocation<T, L, AL, AV, LFS, AFS, Self, FD, FSA>,
    LFS: LiteralFieldSpec<T, L, AL, AV, S, AFS, FD, FSA, Self>,
    AFS: ArrayFieldSpec<T, L, AL, AV, S, LFS, FD, FSA, Self>,
    FD: FieldDefinition<Self>,
    FSA: FieldSelectionArgument<T, L, AL, AV, S, LFS, AFS, Self, FD>,
>: Sized
{
    fn get_ref(self: &Self) -> NonCallableFieldSpecRef<'_, LFS, AFS>;
    fn has_default_value(self: &Self) -> bool;
}

pub trait FieldDefinition<T> {
    fn get_name(self: &Self) -> &str;
    fn get_spec(self: &Self) -> T;
    fn get_nullable(self: &Self) -> bool;
}

pub enum ArgumentLiteralValueRef<'r> {
    String(&'r str),
    Int(&'r i64),
    Float(&'r f64),
    Boolean(&'r bool),
    EnumValue(&'r str),
}

pub trait ArgumentLiteralValue {
    fn get_ref(self: &Self) -> ArgumentLiteralValueRef<'_>;
}

pub enum ArgumentValueRef<'r> {
    Ref(&'r str),
    Literal(ArgumentLiteralValueRef<'r>),
}

pub trait ArgumentValue {
    fn get_ref(self: &Self) -> ArgumentValueRef<'_>;
}

pub trait FieldSelectionArgument<
    T,
    L: Literal,
    AL: ArrayLiteral,
    AV: ArgumentValue,
    S: ServerDirectiveInvocation<T, L, AL, AV, LFS, AFS, NCFS, FD, Self>,
    LFS: LiteralFieldSpec<T, L, AL, AV, S, AFS, FD, Self, NCFS>,
    AFS: ArrayFieldSpec<T, L, AL, AV, S, LFS, FD, Self, NCFS>,
    NCFS: NonCallableFieldSpec<T, L, AL, AV, S, LFS, AFS, FD, Self>,
    FD: FieldDefinition<NCFS>,
>: Sized
{
    fn get_name(self: &Self) -> &str;
    fn get_value(self: &Self) -> &AV;
    fn get_type(self: &Self) -> &FD;
}

pub trait ServerDirectiveInvocation<
    T,
    L: Literal,
    AL: ArrayLiteral,
    AV: ArgumentValue,
    LFS: LiteralFieldSpec<T, L, AL, AV, Self, AFS, FD, FSA, NCFS>,
    AFS: ArrayFieldSpec<T, L, AL, AV, Self, LFS, FD, FSA, NCFS>,
    NCFS: NonCallableFieldSpec<T, L, AL, AV, Self, LFS, AFS, FD, FSA>,
    FD: FieldDefinition<NCFS>,
    FSA: FieldSelectionArgument<T, L, AL, AV, Self, LFS, AFS, NCFS, FD>,
>: Sized
{
    fn get_directive_name(self: &Self) -> &str;
    fn get_arguments(self: &Self) -> &[(&str, &FSA)];
}
