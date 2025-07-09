// Enhanced stub types for @cliffy/table
declare module '@cliffy/table' {
  export type CellValue = string | number | boolean | null | undefined;
  export type RowValue = CellValue | CellValue[];
  export type TableData = RowValue[];

  export interface CellOptions {
    content?: CellValue;
    colSpan?: number;
    rowSpan?: number;
    align?: 'left' | 'center' | 'right';
    color?: string;
    bgColor?: string;
    border?: boolean;
    padding?: number | [number, number] | [number, number, number, number];
    wordWrap?: boolean;
    noWrap?: boolean;
    wrap?: boolean;
    bold?: boolean;
    italic?: boolean;
    underline?: boolean;
    strikethrough?: boolean;
    inverse?: boolean;
    dim?: boolean;
    hidden?: boolean;
    maxWidth?: number;
    minWidth?: number;
    width?: number;
    height?: number;
    maxHeight?: number;
    minHeight?: number;
    verticalAlign?: 'top' | 'middle' | 'bottom';
    clip?: boolean;
    ellipsis?: string;
    transform?: (value: CellValue) => CellValue;
    getValue?: () => CellValue;
    setValue?: (value: CellValue) => void;
    clone?: () => Cell;
    toString?: () => string;
    valueOf?: () => CellValue;
    length?: number;
    isEmpty?: boolean;
    isSpanned?: boolean;
    isHeader?: boolean;
    isFooter?: boolean;
    isBody?: boolean;
    getColSpan?: () => number;
    getRowSpan?: () => number;
    getAlign?: () => string;
    getColor?: () => string;
    getBgColor?: () => string;
    getBorder?: () => boolean;
    getPadding?: () => number | [number, number] | [number, number, number, number];
    getWordWrap?: () => boolean;
    getNoWrap?: () => boolean;
    getWrap?: () => boolean;
    getBold?: () => boolean;
    getItalic?: () => boolean;
    getUnderline?: () => boolean;
    getStrikethrough?: () => boolean;
    getInverse?: () => boolean;
    getDim?: () => boolean;
    getHidden?: () => boolean;
    getMaxWidth?: () => number;
    getMinWidth?: () => number;
    getWidth?: () => number;
    getHeight?: () => number;
    getMaxHeight?: () => number;
    getMinHeight?: () => number;
    getVerticalAlign?: () => string;
    getClip?: () => boolean;
    getEllipsis?: () => string;
    getTransform?: () => (value: CellValue) => CellValue;
  }

  export interface RowOptions {
    cells?: (Cell | CellValue)[];
    align?: 'left' | 'center' | 'right';
    color?: string;
    bgColor?: string;
    border?: boolean;
    padding?: number | [number, number] | [number, number, number, number];
    wordWrap?: boolean;
    noWrap?: boolean;
    wrap?: boolean;
    bold?: boolean;
    italic?: boolean;
    underline?: boolean;
    strikethrough?: boolean;
    inverse?: boolean;
    dim?: boolean;
    hidden?: boolean;
    maxWidth?: number;
    minWidth?: number;
    width?: number;
    height?: number;
    maxHeight?: number;
    minHeight?: number;
    verticalAlign?: 'top' | 'middle' | 'bottom';
    separator?: boolean;
    indent?: number;
    clone?: () => Row;
    toString?: () => string;
    valueOf?: () => CellValue[];
    length?: number;
    isEmpty?: boolean;
    isHeader?: boolean;
    isFooter?: boolean;
    isBody?: boolean;
    isSeparator?: boolean;
    getAlign?: () => string;
    getColor?: () => string;
    getBgColor?: () => string;
    getBorder?: () => boolean;
    getPadding?: () => number | [number, number] | [number, number, number, number];
    getWordWrap?: () => boolean;
    getNoWrap?: () => boolean;
    getWrap?: () => boolean;
    getBold?: () => boolean;
    getItalic?: () => boolean;
    getUnderline?: () => boolean;
    getStrikethrough?: () => boolean;
    getInverse?: () => boolean;
    getDim?: () => boolean;
    getHidden?: () => boolean;
    getMaxWidth?: () => number;
    getMinWidth?: () => number;
    getWidth?: () => number;
    getHeight?: () => number;
    getMaxHeight?: () => number;
    getMinHeight?: () => number;
    getVerticalAlign?: () => string;
    getSeparator?: () => boolean;
    getIndent?: () => number;
  }

  export interface BorderOptions {
    top?: string;
    right?: string;
    bottom?: string;
    left?: string;
    topLeft?: string;
    topRight?: string;
    bottomLeft?: string;
    bottomRight?: string;
    topBody?: string;
    rightBody?: string;
    bottomBody?: string;
    leftBody?: string;
    bodyLeft?: string;
    bodyRight?: string;
    bodyJoin?: string;
    joinLeft?: string;
    joinRight?: string;
    joinTop?: string;
    joinBottom?: string;
    joinBody?: string;
    joinJoin?: string;
    headerTop?: string;
    headerBottom?: string;
    headerLeft?: string;
    headerRight?: string;
    headerJoin?: string;
    footerTop?: string;
    footerBottom?: string;
    footerLeft?: string;
    footerRight?: string;
    footerJoin?: string;
    style?: 'single' | 'double' | 'round' | 'bold' | 'ascii' | 'none';
    color?: string;
    enabled?: boolean;
    chars?: {
      [key: string]: string;
    };
    sides?: {
      top?: boolean;
      right?: boolean;
      bottom?: boolean;
      left?: boolean;
    };
    corners?: {
      topLeft?: boolean;
      topRight?: boolean;
      bottomLeft?: boolean;
      bottomRight?: boolean;
    };
    joins?: {
      topBody?: boolean;
      rightBody?: boolean;
      bottomBody?: boolean;
      leftBody?: boolean;
      bodyLeft?: boolean;
      bodyRight?: boolean;
      bodyJoin?: boolean;
      joinLeft?: boolean;
      joinRight?: boolean;
      joinTop?: boolean;
      joinBottom?: boolean;
      joinBody?: boolean;
      joinJoin?: boolean;
    };
    header?: {
      top?: boolean;
      bottom?: boolean;
      left?: boolean;
      right?: boolean;
      join?: boolean;
    };
    footer?: {
      top?: boolean;
      bottom?: boolean;
      left?: boolean;
      right?: boolean;
      join?: boolean;
    };
  }

  export interface TableOptions {
    chars?: BorderOptions['chars'];
    style?: BorderOptions['style'];
    border?: boolean | BorderOptions;
    indent?: number;
    maxColWidth?: number;
    minColWidth?: number;
    padding?: number | [number, number] | [number, number, number, number];
    header?: RowValue[];
    body?: TableData;
    footer?: RowValue[];
    align?: 'left' | 'center' | 'right';
    color?: string;
    bgColor?: string;
    wordWrap?: boolean;
    noWrap?: boolean;
    wrap?: boolean;
    bold?: boolean;
    italic?: boolean;
    underline?: boolean;
    strikethrough?: boolean;
    inverse?: boolean;
    dim?: boolean;
    hidden?: boolean;
    maxWidth?: number;
    minWidth?: number;
    width?: number;
    height?: number;
    maxHeight?: number;
    minHeight?: number;
    verticalAlign?: 'top' | 'middle' | 'bottom';
    columns?: Array<{
      align?: 'left' | 'center' | 'right';
      color?: string;
      bgColor?: string;
      border?: boolean;
      padding?: number | [number, number] | [number, number, number, number];
      wordWrap?: boolean;
      noWrap?: boolean;
      wrap?: boolean;
      bold?: boolean;
      italic?: boolean;
      underline?: boolean;
      strikethrough?: boolean;
      inverse?: boolean;
      dim?: boolean;
      hidden?: boolean;
      maxWidth?: number;
      minWidth?: number;
      width?: number;
      height?: number;
      maxHeight?: number;
      minHeight?: number;
      verticalAlign?: 'top' | 'middle' | 'bottom';
      transform?: (value: CellValue) => CellValue;
      format?: (value: CellValue) => CellValue;
      clip?: boolean;
      ellipsis?: string;
    }>;
    rows?: RowOptions[];
    sort?: boolean | ((a: RowValue, b: RowValue) => number);
    filter?: (row: RowValue, index: number) => boolean;
    map?: (row: RowValue, index: number) => RowValue;
    clone?: () => Table;
    toString?: () => string;
    valueOf?: () => TableData;
    length?: number;
    isEmpty?: boolean;
    getHeader?: () => Row | undefined;
    getBody?: () => Row[];
    getFooter?: () => Row | undefined;
    getRows?: () => Row[];
    getRow?: (index: number) => Row | undefined;
    getCol?: (index: number) => Cell[];
    getCell?: (row: number, col: number) => Cell | undefined;
    getColCount?: () => number;
    getRowCount?: () => number;
    getMaxColWidth?: () => number;
    getMinColWidth?: () => number;
    getPadding?: () => number | [number, number] | [number, number, number, number];
    getAlign?: () => string;
    getColor?: () => string;
    getBgColor?: () => string;
    getBorder?: () => boolean | BorderOptions;
    getIndent?: () => number;
    getWordWrap?: () => boolean;
    getNoWrap?: () => boolean;
    getWrap?: () => boolean;
    getBold?: () => boolean;
    getItalic?: () => boolean;
    getUnderline?: () => boolean;
    getStrikethrough?: () => boolean;
    getInverse?: () => boolean;
    getDim?: () => boolean;
    getHidden?: () => boolean;
    getMaxWidth?: () => number;
    getMinWidth?: () => number;
    getWidth?: () => number;
    getHeight?: () => number;
    getMaxHeight?: () => number;
    getMinHeight?: () => number;
    getVerticalAlign?: () => string;
    getColumns?: () => TableOptions['columns'];
    getSort?: () => boolean | ((a: RowValue, b: RowValue) => number);
    getFilter?: () => (row: RowValue, index: number) => boolean;
    getMap?: () => (row: RowValue, index: number) => RowValue;
  }

  export class Cell {
    constructor(value?: CellValue, options?: CellOptions);
    
    // Static methods
    static from(value: CellValue | Cell, options?: CellOptions): Cell;
    static isCell(value: any): value is Cell;
    
    // Instance methods
    clone(): Cell;
    toString(): string;
    valueOf(): CellValue;
    
    // Getters and setters
    get content(): CellValue;
    set content(value: CellValue);
    get colSpan(): number;
    set colSpan(value: number);
    get rowSpan(): number;
    set rowSpan(value: number);
    get align(): string;
    set align(value: 'left' | 'center' | 'right');
    get color(): string;
    set color(value: string);
    get bgColor(): string;
    set bgColor(value: string);
    get border(): boolean;
    set border(value: boolean);
    get padding(): number | [number, number] | [number, number, number, number];
    set padding(value: number | [number, number] | [number, number, number, number]);
    get wordWrap(): boolean;
    set wordWrap(value: boolean);
    get noWrap(): boolean;
    set noWrap(value: boolean);
    get wrap(): boolean;
    set wrap(value: boolean);
    get bold(): boolean;
    set bold(value: boolean);
    get italic(): boolean;
    set italic(value: boolean);
    get underline(): boolean;
    set underline(value: boolean);
    get strikethrough(): boolean;
    set strikethrough(value: boolean);
    get inverse(): boolean;
    set inverse(value: boolean);
    get dim(): boolean;
    set dim(value: boolean);
    get hidden(): boolean;
    set hidden(value: boolean);
    get maxWidth(): number;
    set maxWidth(value: number);
    get minWidth(): number;
    set minWidth(value: number);
    get width(): number;
    set width(value: number);
    get height(): number;
    set height(value: number);
    get maxHeight(): number;
    set maxHeight(value: number);
    get minHeight(): number;
    set minHeight(value: number);
    get verticalAlign(): string;
    set verticalAlign(value: 'top' | 'middle' | 'bottom');
    get clip(): boolean;
    set clip(value: boolean);
    get ellipsis(): string;
    set ellipsis(value: string);
    get transform(): (value: CellValue) => CellValue;
    set transform(value: (value: CellValue) => CellValue);
    
    // State
    get length(): number;
    get isEmpty(): boolean;
    get isSpanned(): boolean;
    get isHeader(): boolean;
    get isFooter(): boolean;
    get isBody(): boolean;
  }

  export class Row {
    constructor(cells?: (Cell | CellValue)[], options?: RowOptions);
    
    // Static methods
    static from(cells: (Cell | CellValue)[] | Row, options?: RowOptions): Row;
    static isRow(value: any): value is Row;
    
    // Instance methods
    clone(): Row;
    toString(): string;
    valueOf(): CellValue[];
    push(...cells: (Cell | CellValue)[]): this;
    pop(): Cell | undefined;
    shift(): Cell | undefined;
    unshift(...cells: (Cell | CellValue)[]): this;
    slice(start?: number, end?: number): Cell[];
    splice(start: number, deleteCount?: number, ...items: (Cell | CellValue)[]): Cell[];
    indexOf(cell: Cell | CellValue): number;
    includes(cell: Cell | CellValue): boolean;
    forEach(fn: (cell: Cell, index: number) => void): void;
    map<T>(fn: (cell: Cell, index: number) => T): T[];
    filter(fn: (cell: Cell, index: number) => boolean): Cell[];
    reduce<T>(fn: (acc: T, cell: Cell, index: number) => T, initial: T): T;
    find(fn: (cell: Cell, index: number) => boolean): Cell | undefined;
    findIndex(fn: (cell: Cell, index: number) => boolean): number;
    some(fn: (cell: Cell, index: number) => boolean): boolean;
    every(fn: (cell: Cell, index: number) => boolean): boolean;
    sort(fn?: (a: Cell, b: Cell) => number): this;
    reverse(): this;
    
    // Array-like access
    [index: number]: Cell;
    
    // Getters and setters
    get cells(): Cell[];
    set cells(value: (Cell | CellValue)[]);
    get align(): string;
    set align(value: 'left' | 'center' | 'right');
    get color(): string;
    set color(value: string);
    get bgColor(): string;
    set bgColor(value: string);
    get border(): boolean;
    set border(value: boolean);
    get padding(): number | [number, number] | [number, number, number, number];
    set padding(value: number | [number, number] | [number, number, number, number]);
    get wordWrap(): boolean;
    set wordWrap(value: boolean);
    get noWrap(): boolean;
    set noWrap(value: boolean);
    get wrap(): boolean;
    set wrap(value: boolean);
    get bold(): boolean;
    set bold(value: boolean);
    get italic(): boolean;
    set italic(value: boolean);
    get underline(): boolean;
    set underline(value: boolean);
    get strikethrough(): boolean;
    set strikethrough(value: boolean);
    get inverse(): boolean;
    set inverse(value: boolean);
    get dim(): boolean;
    set dim(value: boolean);
    get hidden(): boolean;
    set hidden(value: boolean);
    get maxWidth(): number;
    set maxWidth(value: number);
    get minWidth(): number;
    set minWidth(value: number);
    get width(): number;
    set width(value: number);
    get height(): number;
    set height(value: number);
    get maxHeight(): number;
    set maxHeight(value: number);
    get minHeight(): number;
    set minHeight(value: number);
    get verticalAlign(): string;
    set verticalAlign(value: 'top' | 'middle' | 'bottom');
    get separator(): boolean;
    set separator(value: boolean);
    get indent(): number;
    set indent(value: number);
    
    // State
    get length(): number;
    get isEmpty(): boolean;
    get isHeader(): boolean;
    get isFooter(): boolean;
    get isBody(): boolean;
    get isSeparator(): boolean;
  }

  export class Table {
    constructor(data?: TableData | Table, options?: TableOptions);
    
    // Static methods
    static from(data: TableData | Table, options?: TableOptions): Table;
    static isTable(value: any): value is Table;
    
    // Configuration methods
    header(headers: RowValue[]): this;
    header(header: Row): this;
    body(rows: TableData): this;
    body(...rows: RowValue[]): this;
    footer(footers: RowValue[]): this;
    footer(footer: Row): this;
    border(enabled?: boolean): this;
    border(options: BorderOptions): this;
    chars(chars: BorderOptions['chars']): this;
    style(style: BorderOptions['style']): this;
    indent(indent: number): this;
    maxColWidth(width: number): this;
    minColWidth(width: number): this;
    padding(padding: number | [number, number] | [number, number, number, number]): this;
    align(align: 'left' | 'center' | 'right'): this;
    color(color: string): this;
    bgColor(color: string): this;
    wordWrap(enabled?: boolean): this;
    noWrap(enabled?: boolean): this;
    wrap(enabled?: boolean): this;
    bold(enabled?: boolean): this;
    italic(enabled?: boolean): this;
    underline(enabled?: boolean): this;
    strikethrough(enabled?: boolean): this;
    inverse(enabled?: boolean): this;
    dim(enabled?: boolean): this;
    hidden(enabled?: boolean): this;
    maxWidth(width: number): this;
    minWidth(width: number): this;
    width(width: number): this;
    height(height: number): this;
    maxHeight(height: number): this;
    minHeight(height: number): this;
    verticalAlign(align: 'top' | 'middle' | 'bottom'): this;
    columns(columns: TableOptions['columns']): this;
    sort(enabled?: boolean): this;
    sort(fn: (a: RowValue, b: RowValue) => number): this;
    filter(fn: (row: RowValue, index: number) => boolean): this;
    map(fn: (row: RowValue, index: number) => RowValue): this;
    
    // Data manipulation methods
    push(...rows: RowValue[]): this;
    push(...rows: Row[]): this;
    pop(): Row | undefined;
    shift(): Row | undefined;
    unshift(...rows: RowValue[]): this;
    unshift(...rows: Row[]): this;
    slice(start?: number, end?: number): Row[];
    splice(start: number, deleteCount?: number, ...items: RowValue[]): Row[];
    splice(start: number, deleteCount?: number, ...items: Row[]): Row[];
    indexOf(row: Row | RowValue): number;
    includes(row: Row | RowValue): boolean;
    forEach(fn: (row: Row, index: number) => void): void;
    reduce<T>(fn: (acc: T, row: Row, index: number) => T, initial: T): T;
    find(fn: (row: Row, index: number) => boolean): Row | undefined;
    findIndex(fn: (row: Row, index: number) => boolean): number;
    some(fn: (row: Row, index: number) => boolean): boolean;
    every(fn: (row: Row, index: number) => boolean): boolean;
    reverse(): this;
    
    // Array-like access
    [index: number]: Row;
    
    // Rendering methods
    render(): string;
    toString(): string;
    valueOf(): TableData;
    
    // Instance methods
    clone(): Table;
    clear(): this;
    
    // Getters
    get length(): number;
    get isEmpty(): boolean;
    get header(): Row | undefined;
    get body(): Row[];
    get footer(): Row | undefined;
    get rows(): Row[];
    get colCount(): number;
    get rowCount(): number;
    get maxColWidth(): number;
    get minColWidth(): number;
    get padding(): number | [number, number] | [number, number, number, number];
    get align(): string;
    get color(): string;
    get bgColor(): string;
    get border(): boolean | BorderOptions;
    get indent(): number;
    get wordWrap(): boolean;
    get noWrap(): boolean;
    get wrap(): boolean;
    get bold(): boolean;
    get italic(): boolean;
    get underline(): boolean;
    get strikethrough(): boolean;
    get inverse(): boolean;
    get dim(): boolean;
    get hidden(): boolean;
    get maxWidth(): number;
    get minWidth(): number;
    get width(): number;
    get height(): number;
    get maxHeight(): number;
    get minHeight(): number;
    get verticalAlign(): string;
    get columns(): TableOptions['columns'];
    
    // Access methods
    getRow(index: number): Row | undefined;
    getCol(index: number): Cell[];
    getCell(row: number, col: number): Cell | undefined;
    setRow(index: number, row: RowValue | Row): this;
    setCol(index: number, cells: CellValue[] | Cell[]): this;
    setCell(row: number, col: number, cell: CellValue | Cell): this;
  }

  // Utility functions
  export function table(data?: TableData, options?: TableOptions): Table;
  export function row(cells?: (Cell | CellValue)[], options?: RowOptions): Row;
  export function cell(value?: CellValue, options?: CellOptions): Cell;
  
  // Border styles
  export const borderStyles: {
    single: BorderOptions['chars'];
    double: BorderOptions['chars'];
    round: BorderOptions['chars'];
    bold: BorderOptions['chars'];
    ascii: BorderOptions['chars'];
    none: BorderOptions['chars'];
  };
  
  // Default options
  export const defaultTableOptions: TableOptions;
  export const defaultRowOptions: RowOptions;
  export const defaultCellOptions: CellOptions;
  export const defaultBorderOptions: BorderOptions;
  
  // Type guards
  export function isTable(value: any): value is Table;
  export function isRow(value: any): value is Row;
  export function isCell(value: any): value is Cell;
  export function isCellValue(value: any): value is CellValue;
  export function isRowValue(value: any): value is RowValue;
  export function isTableData(value: any): value is TableData;
  
  // Utility types
  export type TableLike = Table | TableData | RowValue[];
  export type RowLike = Row | RowValue | CellValue[];
  export type CellLike = Cell | CellValue;
  
  // Constants
  export const TABLE_EVENTS: {
    BEFORE_RENDER: string;
    AFTER_RENDER: string;
    BEFORE_FORMAT: string;
    AFTER_FORMAT: string;
    CELL_RENDER: string;
    ROW_RENDER: string;
    HEADER_RENDER: string;
    FOOTER_RENDER: string;
    BORDER_RENDER: string;
  };
  
  export const CELL_EVENTS: {
    BEFORE_RENDER: string;
    AFTER_RENDER: string;
    BEFORE_FORMAT: string;
    AFTER_FORMAT: string;
    CONTENT_CHANGE: string;
    STYLE_CHANGE: string;
  };
  
  export const ROW_EVENTS: {
    BEFORE_RENDER: string;
    AFTER_RENDER: string;
    BEFORE_FORMAT: string;
    AFTER_FORMAT: string;
    CELL_ADD: string;
    CELL_REMOVE: string;
    CELL_CHANGE: string;
    STYLE_CHANGE: string;
  };
}