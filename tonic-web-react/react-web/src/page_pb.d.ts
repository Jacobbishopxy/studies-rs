import * as jspb from 'google-protobuf'

import * as google_protobuf_timestamp_pb from 'google-protobuf/google/protobuf/timestamp_pb';


export class PageRequest extends jspb.Message {
  getName(): string;
  setName(value: string): PageRequest;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): PageRequest.AsObject;
  static toObject(includeInstance: boolean, msg: PageRequest): PageRequest.AsObject;
  static serializeBinaryToWriter(message: PageRequest, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): PageRequest;
  static deserializeBinaryFromReader(message: PageRequest, reader: jspb.BinaryReader): PageRequest;
}

export namespace PageRequest {
  export type AsObject = {
    name: string,
  }
}

export class PageReply extends jspb.Message {
  getName(): string;
  setName(value: string): PageReply;

  getTime(): google_protobuf_timestamp_pb.Timestamp | undefined;
  setTime(value?: google_protobuf_timestamp_pb.Timestamp): PageReply;
  hasTime(): boolean;
  clearTime(): PageReply;

  getBlocksList(): Array<Block>;
  setBlocksList(value: Array<Block>): PageReply;
  clearBlocksList(): PageReply;
  addBlocks(value?: Block, index?: number): Block;

  getVersion(): string;
  setVersion(value: string): PageReply;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): PageReply.AsObject;
  static toObject(includeInstance: boolean, msg: PageReply): PageReply.AsObject;
  static serializeBinaryToWriter(message: PageReply, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): PageReply;
  static deserializeBinaryFromReader(message: PageReply, reader: jspb.BinaryReader): PageReply;
}

export namespace PageReply {
  export type AsObject = {
    name: string,
    time?: google_protobuf_timestamp_pb.Timestamp.AsObject,
    blocksList: Array<Block.AsObject>,
    version: string,
  }
}

export class Block extends jspb.Message {
  getParagraphblock(): ParagraphBlock | undefined;
  setParagraphblock(value?: ParagraphBlock): Block;
  hasParagraphblock(): boolean;
  clearParagraphblock(): Block;

  getHeaderblock(): HeaderBlock | undefined;
  setHeaderblock(value?: HeaderBlock): Block;
  hasHeaderblock(): boolean;
  clearHeaderblock(): Block;

  getListblock(): ListBlock | undefined;
  setListblock(value?: ListBlock): Block;
  hasListblock(): boolean;
  clearListblock(): Block;

  getBlockCase(): Block.BlockCase;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Block.AsObject;
  static toObject(includeInstance: boolean, msg: Block): Block.AsObject;
  static serializeBinaryToWriter(message: Block, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Block;
  static deserializeBinaryFromReader(message: Block, reader: jspb.BinaryReader): Block;
}

export namespace Block {
  export type AsObject = {
    paragraphblock?: ParagraphBlock.AsObject,
    headerblock?: HeaderBlock.AsObject,
    listblock?: ListBlock.AsObject,
  }

  export enum BlockCase { 
    BLOCK_NOT_SET = 0,
    PARAGRAPHBLOCK = 1,
    HEADERBLOCK = 2,
    LISTBLOCK = 3,
  }
}

export class ParagraphBlock extends jspb.Message {
  getId(): string;
  setId(value: string): ParagraphBlock;

  getType(): string;
  setType(value: string): ParagraphBlock;

  getData(): ParagraphData | undefined;
  setData(value?: ParagraphData): ParagraphBlock;
  hasData(): boolean;
  clearData(): ParagraphBlock;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ParagraphBlock.AsObject;
  static toObject(includeInstance: boolean, msg: ParagraphBlock): ParagraphBlock.AsObject;
  static serializeBinaryToWriter(message: ParagraphBlock, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ParagraphBlock;
  static deserializeBinaryFromReader(message: ParagraphBlock, reader: jspb.BinaryReader): ParagraphBlock;
}

export namespace ParagraphBlock {
  export type AsObject = {
    id: string,
    type: string,
    data?: ParagraphData.AsObject,
  }
}

export class HeaderBlock extends jspb.Message {
  getId(): string;
  setId(value: string): HeaderBlock;

  getType(): string;
  setType(value: string): HeaderBlock;

  getData(): HeaderData | undefined;
  setData(value?: HeaderData): HeaderBlock;
  hasData(): boolean;
  clearData(): HeaderBlock;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): HeaderBlock.AsObject;
  static toObject(includeInstance: boolean, msg: HeaderBlock): HeaderBlock.AsObject;
  static serializeBinaryToWriter(message: HeaderBlock, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): HeaderBlock;
  static deserializeBinaryFromReader(message: HeaderBlock, reader: jspb.BinaryReader): HeaderBlock;
}

export namespace HeaderBlock {
  export type AsObject = {
    id: string,
    type: string,
    data?: HeaderData.AsObject,
  }
}

export class ListBlock extends jspb.Message {
  getId(): string;
  setId(value: string): ListBlock;

  getType(): string;
  setType(value: string): ListBlock;

  getData(): ListData | undefined;
  setData(value?: ListData): ListBlock;
  hasData(): boolean;
  clearData(): ListBlock;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ListBlock.AsObject;
  static toObject(includeInstance: boolean, msg: ListBlock): ListBlock.AsObject;
  static serializeBinaryToWriter(message: ListBlock, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ListBlock;
  static deserializeBinaryFromReader(message: ListBlock, reader: jspb.BinaryReader): ListBlock;
}

export namespace ListBlock {
  export type AsObject = {
    id: string,
    type: string,
    data?: ListData.AsObject,
  }
}

export class ParagraphData extends jspb.Message {
  getText(): string;
  setText(value: string): ParagraphData;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ParagraphData.AsObject;
  static toObject(includeInstance: boolean, msg: ParagraphData): ParagraphData.AsObject;
  static serializeBinaryToWriter(message: ParagraphData, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ParagraphData;
  static deserializeBinaryFromReader(message: ParagraphData, reader: jspb.BinaryReader): ParagraphData;
}

export namespace ParagraphData {
  export type AsObject = {
    text: string,
  }
}

export class HeaderData extends jspb.Message {
  getText(): string;
  setText(value: string): HeaderData;

  getLevel(): number;
  setLevel(value: number): HeaderData;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): HeaderData.AsObject;
  static toObject(includeInstance: boolean, msg: HeaderData): HeaderData.AsObject;
  static serializeBinaryToWriter(message: HeaderData, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): HeaderData;
  static deserializeBinaryFromReader(message: HeaderData, reader: jspb.BinaryReader): HeaderData;
}

export namespace HeaderData {
  export type AsObject = {
    text: string,
    level: number,
  }
}

export class ListData extends jspb.Message {
  getStyle(): ListStyle;
  setStyle(value: ListStyle): ListData;

  getItemsList(): Array<string>;
  setItemsList(value: Array<string>): ListData;
  clearItemsList(): ListData;
  addItems(value: string, index?: number): ListData;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ListData.AsObject;
  static toObject(includeInstance: boolean, msg: ListData): ListData.AsObject;
  static serializeBinaryToWriter(message: ListData, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ListData;
  static deserializeBinaryFromReader(message: ListData, reader: jspb.BinaryReader): ListData;
}

export namespace ListData {
  export type AsObject = {
    style: ListStyle,
    itemsList: Array<string>,
  }
}

export enum ListStyle { 
  UNORDERED = 0,
  ORDERED = 1,
}
