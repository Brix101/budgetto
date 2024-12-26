import type { TestingModule } from "@nestjs/testing";
import {
  BadRequestException,
  InternalServerErrorException,
  NotFoundException,
} from "@nestjs/common";
import { Test } from "@nestjs/testing";

import { UserRepository } from "./users.repository";
import { UsersService } from "./users.service";

describe("UsersService", () => {
  let service: UsersService;
  let repo: UserRepository;

  beforeEach(async () => {
    const module: TestingModule = await Test.createTestingModule({
      providers: [
        UsersService,
        {
          provide: UserRepository,
          useValue: {
            create: jest.fn(),
            insert: jest.fn().mockResolvedValue(1),
            findOneOrFail: jest.fn(),
            nativeUpdate: jest.fn(),
          },
        },
      ],
    }).compile();

    service = module.get<UsersService>(UsersService);
    repo = module.get<UserRepository>(UserRepository);
  });

  it("should be defined", () => {
    expect(service).toBeDefined();
  });

  describe("create", () => {
    it("should create a user", async () => {
      const createUserDto = {
        name: "John Doe",
        email: "test@example.com",
        password: "password",
      };
      const user = { ...createUserDto, id: 1 };
      (repo.create as jest.Mock).mockReturnValue(user);
      (repo.insert as jest.Mock).mockResolvedValue(user);

      expect(await service.create(createUserDto)).toEqual(user);
    });

    it("should throw BadRequestException if email already exists", async () => {
      const createUserDto = {
        name: "John Doe",
        email: "test@example.com",
        password: "password",
      };
      (repo.create as jest.Mock).mockReturnValue(createUserDto);

      await expect(service.create(createUserDto)).rejects.toThrow(
        BadRequestException,
      );
    });

    it("should throw InternalServerErrorException on other errors", async () => {
      const createUserDto = {
        name: "John Doe",
        email: "test@example.com",
        password: "password",
      };
      (repo.create as jest.Mock).mockReturnValue(createUserDto);
      (repo.insert as jest.Mock).mockRejectedValue(
        new Error("Unexpected error"),
      );

      await expect(service.create(createUserDto)).rejects.toThrow(
        InternalServerErrorException,
      );
    });
  });

  describe("findOne", () => {
    it("should find a user by id", async () => {
      const user = { id: 1, email: "test@example.com" };
      (repo.findOneOrFail as jest.Mock).mockResolvedValue(user);

      expect(await service.findOne({ id: 1 })).toEqual(user);
    });

    it("should throw NotFoundException if user not found", async () => {
      (repo.findOneOrFail as jest.Mock).mockRejectedValue(
        new Error("User not found"),
      );

      await expect(service.findOne({ id: 1 })).rejects.toThrow(
        NotFoundException,
      );
    });
  });

  describe("update", () => {
    it("should update a user", async () => {
      const updateUserDto = { email: "updated@example.com" };
      const user = { id: 1, email: "test@example.com", assign: jest.fn() };
      (repo.findOneOrFail as jest.Mock).mockResolvedValue(user);
      (repo.nativeUpdate as jest.Mock).mockResolvedValue(user);

      expect(await service.update(1, updateUserDto)).toEqual(user);
    });

    it("should throw NotFoundException if user not found", async () => {
      (repo.findOneOrFail as jest.Mock).mockRejectedValue(
        new NotFoundException(),
      );

      await expect(
        service.update(1, { email: "updated@example.com" }),
      ).rejects.toThrow(NotFoundException);
    });

    it("should throw InternalServerErrorException on other errors", async () => {
      const updateUserDto = { email: "updated@example.com" };
      const user = { id: 1, email: "test@example.com", assign: jest.fn() };
      (repo.findOneOrFail as jest.Mock).mockResolvedValue(user);
      (repo.nativeUpdate as jest.Mock).mockRejectedValue(
        new Error("Unexpected error"),
      );

      await expect(service.update(1, updateUserDto)).rejects.toThrow(
        InternalServerErrorException,
      );
    });
  });
});
