import {
  ArgumentsHost,
  Catch,
  ExceptionFilter,
  HttpException,
} from "@nestjs/common";
import { Request, Response } from "express";

@Catch(HttpException)
export class HttpExceptionFilter implements ExceptionFilter {
  catch(exception: HttpException, host: ArgumentsHost) {
    const ctx = host.switchToHttp();
    const response = ctx.getResponse<Response>();
    const request = ctx.getRequest<Request>();
    const status = exception.getStatus();
    const exceptionResponse = exception.getResponse() as {
      error: string;
      message: string | string[];
      errors?: [Record<string, any>];
    };

    const errorResponse: Record<string, any> = {
      timestamp: new Date().toISOString(),
      path: request.url,
    };

    if (typeof exceptionResponse === "string") {
      errorResponse.message = exceptionResponse;
    } else {
      errorResponse.message = exceptionResponse.error;
      errorResponse.errors = exceptionResponse.message;
    }

    // ? Catch errors on ZodpipeValidation
    if (exceptionResponse.errors) {
      errorResponse.message = "Bad Request";
      errorResponse.errors = exceptionResponse.errors;
    }

    response.status(status).json(errorResponse);
  }
}
