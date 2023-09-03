/**
 * @fileoverview gRPC-Web generated client stub for page
 * @enhanceable
 * @public
 */

// Code generated by protoc-gen-grpc-web. DO NOT EDIT.
// versions:
// 	protoc-gen-grpc-web v1.4.2
// 	protoc              v3.20.3
// source: page.proto


/* eslint-disable */
// @ts-nocheck


import * as grpcWeb from 'grpc-web';

import * as page_pb from './page_pb';


export class PageClient {
  client_: grpcWeb.AbstractClientBase;
  hostname_: string;
  credentials_: null | { [index: string]: string; };
  options_: null | { [index: string]: any; };

  constructor (hostname: string,
               credentials?: null | { [index: string]: string; },
               options?: null | { [index: string]: any; }) {
    if (!options) options = {};
    if (!credentials) credentials = {};
    options['format'] = 'binary';

    this.client_ = new grpcWeb.GrpcWebClientBase(options);
    this.hostname_ = hostname.replace(/\/+$/, '');
    this.credentials_ = credentials;
    this.options_ = options;
  }

  methodDescriptorGetPage = new grpcWeb.MethodDescriptor(
    '/page.Page/GetPage',
    grpcWeb.MethodType.UNARY,
    page_pb.PageRequest,
    page_pb.PageReply,
    (request: page_pb.PageRequest) => {
      return request.serializeBinary();
    },
    page_pb.PageReply.deserializeBinary
  );

  getPage(
    request: page_pb.PageRequest,
    metadata: grpcWeb.Metadata | null): Promise<page_pb.PageReply>;

  getPage(
    request: page_pb.PageRequest,
    metadata: grpcWeb.Metadata | null,
    callback: (err: grpcWeb.RpcError,
               response: page_pb.PageReply) => void): grpcWeb.ClientReadableStream<page_pb.PageReply>;

  getPage(
    request: page_pb.PageRequest,
    metadata: grpcWeb.Metadata | null,
    callback?: (err: grpcWeb.RpcError,
               response: page_pb.PageReply) => void) {
    if (callback !== undefined) {
      return this.client_.rpcCall(
        this.hostname_ +
          '/page.Page/GetPage',
        request,
        metadata || {},
        this.methodDescriptorGetPage,
        callback);
    }
    return this.client_.unaryCall(
    this.hostname_ +
      '/page.Page/GetPage',
    request,
    metadata || {},
    this.methodDescriptorGetPage);
  }

}
