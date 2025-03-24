<?php

namespace App\Controller;

use Symfony\Component\HttpFoundation\JsonResponse;
use Symfony\Component\HttpFoundation\Request;
use Symfony\Component\Routing\Annotation\Route;

class BenchyController
{
    #[Route('/', methods: ['POST'])]
    public function handleRequest(Request $request): JsonResponse
    {
        $data = json_decode($request->getContent(), true);

        return new JsonResponse([
            'success' => true,
            'server_time' => (new \DateTime())->format(\DateTime::RFC3339)
        ]);
    }
}